// The MIT License
// 
// Copyright (c) 2024 MGTheTrain
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// 
// Maintainers:
// - MGTheTrain 
// 
// Contributors:
// - TBD

use uuid::Uuid;

pub struct UploadFileParameters {
    pub blob_name: String,
    pub file_name: String,
}

impl UploadFileParameters {
    pub fn new() -> Self {
        UploadFileParameters {
            blob_name: String::from(""),
            file_name: String::from(""),
        }
    }
}

pub struct UploadBytesParameters {
    pub blob_name: String,
    pub file_name: String,
    pub bytes: Vec<u8>,
}

impl UploadBytesParameters {
    pub fn new() -> Self {
        UploadBytesParameters {
            blob_name: String::from(""),
            file_name: String::from(""),
            bytes: Vec::new(),
        }
    }
}

pub struct UploadMetaParameters {
    pub title: String,
    pub description: String,
    pub tags: Vec<Option<String>>,
}

impl UploadMetaParameters {
    pub fn new() -> Self {
        UploadMetaParameters { 
            title: String::from(""), 
            description: String::from(""), 
            tags: Vec::new(), 
        }
    }
}