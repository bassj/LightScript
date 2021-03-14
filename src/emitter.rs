use webassembly;
use webassembly::TypeWasmExt;


pub struct ModuleEmitter;

trait Encoded {
    fn encode(&self) -> Vec<u8>;
}

struct EncodedVec {
    vector: Vec<u8>
}

impl Encoded for EncodedVec {
    fn encode(&self) -> Vec<u8> {
        let len = self.vector.len() as u32;

        let mut encoded: Vec<u8> = Vec::new();

        encoded.extend_from_slice(&len.to_wasm_bytes());
        encoded.extend_from_slice(&self.vector);

        encoded
    }
}

struct EncodedSection {
    section_type: u8,
    section_content: EncodedVec
}

impl Encoded for EncodedSection {
    fn encode(&self) -> Vec<u8> {
        let mut section: Vec<u8> = Vec::new();
        section.push(self.section_type);
        let content = self.section_content.encode();
        section.extend_from_slice(&content);
        section
    }
}

struct EncodedFuncBody {
    locals: Vec<EncodedLocal>,
    instructions: Vec<u8>,
}

impl Encoded for EncodedFuncBody {
    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }
}

struct EncodedLocal {
    count: usize,
    local_type: u8,
}

impl Encoded for EncodedLocal {
    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl ModuleEmitter {
    fn encode_vector(&self, vector: Vec<u8>) -> EncodedVec {
        EncodedVec { vector }
    }
    
    fn encode_section(&self, section_type: u8, section_content: EncodedVec ) -> EncodedSection {
        EncodedSection { section_type, section_content }
    }

    fn encode_function(&self, locals: Vec<EncodedLocal>, instructions: Vec<u8>) -> EncodedFuncBody {
        EncodedFuncBody {
            locals,
            instructions
        }
    }

    fn encode_local(&self, local_type: u8, count: usize) -> EncodedLocal {
        EncodedLocal { count, local_type }
    }

    pub fn emit(&self) -> Vec<u8> {
        let mut prog = webassembly::MAGIC_NUMBER.to_vec();
        prog.extend_from_slice(webassembly::VERSION_1); // Module header


        // We take 2 f32's and return 1, so our locals can just be 3 f32
        let locals = self.encode_local(webassembly::F32, 3);

        let fn_body: Vec<u8> = Vec::new();

        let add_func = self.encode_function(vec![locals], fn_body);
        
        
        //Lets create a basic add function.
        /*let mut add_func: Vec<u8> = Vec::new();


        let mut locals: Vec<u8> = Vec::new();        
        locals.push

        let mut code: Vec<u8> = Vec::new();
        code.push(webassembly::END);



        //Lets export our add function
        let mut export: Vec<u8> = Vec::new();*/
        

        prog
    }
}
