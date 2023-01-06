mod parse;

use std::fs::File;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use cranelift::codegen::ir::immediates::Offset32;
use cranelift::{
    codegen::{
        ir::{Function, UserFuncName},
        Context,
    },
    prelude::{isa::CallConv, types::I32, *},
};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use parse::Program;
use target_lexicon::triple;

use structopt::StructOpt;

use crate::parse::Token;

#[derive(Debug, StructOpt)]
#[structopt(name = "bfrsc")]
struct Opt {
    /// The input brainfuck file
    input: PathBuf,
    /// Where to output to. Default is `a.out`
    output: Option<PathBuf>,
}

struct CodeGen {
    program: Program,
}

impl CodeGen {
    fn new(program: Program) -> Self {
        Self { program }
    }

    fn compile(&self, output: impl AsRef<Path>) {
        let mut shared_builder = settings::builder();
        shared_builder.enable("is_pic").unwrap();
        let shared_flags = settings::Flags::new(shared_builder);

        let isa_builder = isa::lookup(triple!("x86_64-unknown-linux-gnu")).unwrap();
        let isa = isa_builder.finish(shared_flags).unwrap();

        let obj_builder =
            ObjectBuilder::new(isa, "main", cranelift_module::default_libcall_names()).unwrap();
        let mut obj_module = ObjectModule::new(obj_builder);

        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(I32));
        let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
        let mut func_ctx = FunctionBuilderContext::new();
        let mut fn_builder = FunctionBuilder::new(&mut func, &mut func_ctx);

        // Global Memory Definition
        let mem_array = obj_module
            .declare_data("mem", Linkage::Local, true, false)
            .unwrap();
        let mut data_ctx = DataContext::new();
        data_ctx.define_zeroinit(30000);
        obj_module.define_data(mem_array, &data_ctx).unwrap();
        let local_data = obj_module.declare_data_in_func(mem_array, fn_builder.func);
        let pointer = obj_module.target_config().pointer_type();

        // Putchar Definition
        let mut putchar_sig = obj_module.make_signature();
        putchar_sig.params.push(AbiParam::new(types::I8));
        putchar_sig.returns.push(AbiParam::new(types::I32));
        let putchar_fn = obj_module
            .declare_function("putchar", Linkage::Import, &putchar_sig)
            .unwrap();
        let local_putchar = obj_module.declare_func_in_func(putchar_fn, fn_builder.func);

        // Getchar Definition
        let mut getchar_sig = obj_module.make_signature();
        getchar_sig.returns.push(AbiParam::new(types::I8));
        let getchar_fn = obj_module
            .declare_function("getchar", Linkage::Import, &getchar_sig)
            .unwrap();
        let local_getchar = obj_module.declare_func_in_func(getchar_fn, fn_builder.func);

        let block = fn_builder.create_block();
        fn_builder.switch_to_block(block);
        fn_builder.seal_block(block);

        let mem = fn_builder.ins().global_value(pointer, local_data);
        let zero = fn_builder.ins().iconst(types::I32, 0);
        let zero_ptr = fn_builder.ins().iconst(pointer, 0);
        let errno = Variable::new(0);
        fn_builder.declare_var(errno, types::I32);
        fn_builder.def_var(errno, zero);
        let offset = Variable::new(1);
        fn_builder.declare_var(offset, pointer);
        fn_builder.def_var(offset, zero_ptr);
        let mut jumps = Vec::new();
        for instr in &self.program.tokens {
            match instr {
                Token::Right(count) => {
                    //let val = fn_builder.ins().iadd(mem, one_ptr);
                    //fn_builder.ins().store(MemFlags::new(), val, p, Offset)
                    let offset_val = fn_builder.use_var(offset);
                    let to_add = fn_builder.ins().iconst(pointer, *count as i64);
                    let new_offset = fn_builder.ins().iadd(offset_val, to_add);
                    fn_builder.def_var(offset, new_offset);
                }
                Token::Left(count) => {
                    let offset_val = fn_builder.use_var(offset);
                    let to_add = fn_builder.ins().iconst(pointer, *count as i64);
                    let new_offset = fn_builder.ins().isub(offset_val, to_add);
                    fn_builder.def_var(offset, new_offset);
                }
                Token::Increment(count) => {
                    let offset_val = fn_builder.use_var(offset);
                    let mem_offset = fn_builder.ins().iadd(mem, offset_val);
                    let val = fn_builder.ins().load(
                        types::I8,
                        MemFlags::new(),
                        mem_offset,
                        Offset32::new(0),
                    );
                    let to_add = fn_builder.ins().iconst(types::I8, *count as i64);
                    let val = fn_builder.ins().iadd(val, to_add);
                    fn_builder
                        .ins()
                        .store(MemFlags::new(), val, mem_offset, Offset32::new(0));
                }
                Token::Decrement(count) => {
                    let offset_val = fn_builder.use_var(offset);
                    let mem_offset = fn_builder.ins().iadd(mem, offset_val);
                    let val = fn_builder.ins().load(
                        types::I8,
                        MemFlags::new(),
                        mem_offset,
                        Offset32::new(0),
                    );
                    let to_add = fn_builder.ins().iconst(types::I8, *count as i64);
                    let val = fn_builder.ins().isub(val, to_add);
                    fn_builder
                        .ins()
                        .store(MemFlags::new(), val, mem_offset, Offset32::new(0));
                }
                Token::Output => {
                    let offset_val = fn_builder.use_var(offset);
                    let mem_offset = fn_builder.ins().iadd(mem, offset_val);
                    let val = fn_builder.ins().load(
                        types::I8,
                        MemFlags::new(),
                        mem_offset,
                        Offset32::new(0),
                    );
                    let ret = fn_builder.ins().call(local_putchar, &[val]);
                    let e = fn_builder.inst_results(ret)[0];
                    fn_builder.def_var(errno, e);
                }
                Token::Input => {
                    let offset_val = fn_builder.use_var(offset);
                    let mem_offset = fn_builder.ins().iadd(mem, offset_val);
                    let res = fn_builder.ins().call(local_getchar, &[]);
                    let val = fn_builder.inst_results(res)[0];
                    fn_builder
                        .ins()
                        .store(MemFlags::new(), val, mem_offset, Offset32::new(0));
                }
                Token::LoopStart => {
                    let next_block = fn_builder.create_block();
                    fn_builder.ins().jump(next_block, &[]);
                    jumps.push(next_block);
                    fn_builder.switch_to_block(next_block);
                }
                Token::LoopEnd => {
                    let val =
                        fn_builder
                            .ins()
                            .load(types::I8, MemFlags::new(), mem, Offset32::new(0));
                    let jump_target = jumps.pop().unwrap();
                    fn_builder.ins().brnz(val, jump_target, &[]);
                    let next_block = fn_builder.create_block();
                    fn_builder.ins().jump(next_block, &[]);
                    fn_builder.switch_to_block(next_block);
                }
            }
        }

        let val = fn_builder.ins().iconst(I32, 0);
        //let val = fn_builder.use_var(errno);
        fn_builder.ins().return_(&[val]);
        fn_builder.finalize();

        println!("{}", func.display());

        let mut context = Context::for_function(func);

        let mut sig = obj_module.make_signature();
        sig.returns.push(AbiParam::new(types::I32));
        let fid = obj_module
            .declare_function("main", Linkage::Export, &sig)
            .unwrap();
        obj_module.define_function(fid, &mut context).unwrap();
        let res = obj_module.finish();

        let mut file = File::create(output).unwrap();
        res.object.write_stream(&mut file).unwrap();
    }
}

fn main() -> color_eyre::Result<()> {
    let opt = Opt::from_args();
    let source = std::fs::read_to_string(opt.input)?;
    let program = Program::from(source.as_str());
    //dbg!(&program);
    let codegen = CodeGen::new(program);
    codegen.compile(opt.output.unwrap_or(PathBuf::from("a.out")));
    Ok(())
}