use super::dtos::create_#name#_request::Create#Name#Request;
use super::dtos::create_#name#_response::Create#Name#Response;
use super::dtos::get_#name#_response::Get#Name#Response;
use super::dtos::list_#name#_response::List#Name#Response;
use super::dtos::update_#name#_request::Update#Name#Request;
use super::dtos::update_#name#_response::Update#Name#Response;
use super::dtos::delete_#name#_response::Delete#Name#Response;

pub struct #Name#Service {}

impl #Name#Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_one(&self, _dto: Create#Name#Request) -> Create#Name#Response {
        // create something...
        Create#Name#Response { id: 1 }
    }

    pub fn update_one(&self, _id: i32, _dto: Create#Name#Request) -> Update#Name#Response {
        // update something...
        Create#Name#Response { }
    }

    pub fn delete_one(&self, _id: i32) -> Delete#Name#Response {
        // delete something...
        Delete#Name#Response { }
    }

    pub fn find_one(&self, id: i32) -> Get#Name#Response {
        // find something...
        Get#Name#Response { 
            data: User { 
                id,
            } 
        }
    }

    pub fn find_all(&self) -> List#Name#Response {
        // find something...
        List#Name#Response { 
            list: vec![
                User { 
                    id: 1
                } 
            ] 
        }
    }
}
