use std::io::{BufRead, BufReader, Read};

use gl::types::{GLchar, GLint};

use super::prelude::{Build, Delete, GetId, Status};

#[derive(Debug, Clone)]
pub struct Shader<Src: BufRead> {
    id: u32,
    status: StatusShader,
    src: Src,
}

impl<Src: BufRead + Clone> Shader<Src> {
    pub fn new(type_shader: gl::types::GLenum, src: Src) -> Self {
        unsafe {
            let id = gl::CreateShader(type_shader);
            let mut buf_reader = BufReader::new(src.clone());

            let mut buf = vec![];
            buf_reader.read_to_end(&mut buf).unwrap();

            gl::ShaderSource(
                id,
                1,
                &std::ffi::CString::new(buf).unwrap().as_ptr(),
                std::ptr::null(),
            );

            Self {
                id,
                status: Default::default(),
                src,
            }
        }
    }
}

impl<Src: BufRead> GetId for Shader<Src> {
    fn id(&self) -> u32 {
        self.id
    }
}

impl<Src: BufRead> Status for Shader<Src> {
    type Output = StatusShader;
    fn status(&self) -> Self::Output {
        self.status.clone()
    }
}

impl<Src: BufRead> Read for Shader<Src> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.src.read(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        self.src.read_to_string(buf)
    }
}

impl<Src: BufRead> Build for Shader<Src> {
    fn build(&mut self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id());
            let mut buf = vec![];
            self.read_to_end(&mut buf).unwrap();

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(self.id(), gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(self.id(), gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);

                gl::GetShaderInfoLog(
                    self.id(),
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let err = format!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );

                self.status = StatusShader::ErrorCompile(err.clone());
                return Err(err);
            }

            self.status = StatusShader::CompiledSuccessfully;
            Ok(())
        }
    }
}

impl<Src: BufRead> Delete for Shader<Src> {
    fn delete(self) {
        unsafe {
            gl::DeleteShader(self.id());
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum StatusShader {
    ErrorCompile(String),
    #[default]
    NotCompiled,
    CompiledSuccessfully,
}
