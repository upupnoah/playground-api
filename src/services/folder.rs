// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use uuid::Uuid;

use amp_common::scm::content::Content;

use crate::context::Context;
use crate::errors::Result;

pub struct FolderService;

impl FolderService {
    pub async fn get(_ctx: Arc<Context>, _id: Uuid, _reference: String, _path: String) -> Result<Content> {
        todo!()
    }

    pub async fn create(_ctx: Arc<Context>, _id: Uuid, _reference: String, _path: String) -> Result<Content> {
        todo!()
    }

    pub async fn delete(_ctx: Arc<Context>, _id: Uuid, _reference: String, _path: String) -> Result<()> {
        todo!()
    }

    pub async fn copy(
        _ctx: Arc<Context>,
        _id: Uuid,
        _reference: String,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        todo!()
    }

    pub async fn rename(
        _ctx: Arc<Context>,
        _id: Uuid,
        _reference: String,
        _path: String,
        _destination: String,
    ) -> Result<Content> {
        todo!()
    }
}