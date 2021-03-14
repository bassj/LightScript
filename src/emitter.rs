use webassembly;
use webassembly::TypeWasmExt;

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

struct FuncSection {
    signature_ids: Vec<u32>
}

impl Encoded for FuncSection {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        for id in signature_ids {
            encoded.extend_from_slice(&id.to_wasm_bytes());
        }

        encoded
    }
}

struct EncodedFuncBody {
    locals: Vec<EncodedLocal>,
    instructions: Vec<u8>,
}

impl Encoded for EncodedFuncBody {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();



        encoded
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

struct EncodedModule {
    custom_section: Option<EncodedSection>,
    type_section: Option<EncodedSection>,
    import_section: Option<EncodedSection>,
    function_section: Option<EncodedSection>,
    table_section: Option<EncodedSection>,
    memory_section: Option<EncodedSection>,
    global_section: Option<EncodedSection>,
    export_section: Option<EncodedSection>,
    start_section: Option<EncodedSection>,
    element_section: Option<EncodedSection>,
    code_section: Option<EncodedSection>,
    data_section: Option<EncodedSection>,
    data_count_section: Option<EncodedSection>
}

impl Encoded for EncodedModule {
    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl Default for EncodedModule {
    fn default() -> Self {
        Self {
            custom_section: None,
            type_section: None,
            import_section: None,
            function_section: None,
            table_section: None,
            memory_section: None,
            global_section: None,
            export_section: None,
            start_section: None,
            element_section: None,
            code_section: None,
            data_section: None,
            data_count_section: None
        }
    }
}

pub struct ModuleEmitter;

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
        let mut module = EncodedModule::default();
        // For right now we're just gonna build a simple module that exports a add function that accepts 2 f32's and returns 1 f32
        
        // Lets build the func section.
        let func_section = FuncSection {

        }

        // Lets build the code section. This is the actual implmentation of our add function.
        // We take 2 f32's and return 1, so our locals can just be 3 f32
        let locals = self.encode_local(webassembly::F32, 3);
        let instructions: Vec<u8> = Vec::new();
        let add_func = self.encode_function(vec![locals], instructions);


        let func_vec = self.encode_vector(add_func.encode());
        let code_section = self.encode_section(webassembly::SECTION_CODE, func_vec);
        module.code_section = Some(code_section);

        module.encode()
    }
}
