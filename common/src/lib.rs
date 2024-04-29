/*
 * @Description:
 * @Author: PhilRandWu
 * @Github: https://github/PhilRandWu
 * @Date: 2024-04-28 16:05:46
 * @LastEditTime: 2024-04-28 16:09:16
 * @LastEditors: PhilRandWu
 */
pub struct Owner {
    pub id: i32,
    pub name: String,
}

pub struct OwnerRequest {
    pub name: String,
}

pub struct OwnerResponse {
    pub id: i32,
    pub name: String,
}

impl OwnerResponse {
    pub fn of(owner: Owner) -> OwnerResponse {
        OwnerResponse {
            id: owner.id,
            name: owner.name
        }
    }
}

pub struct Pet {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub animal_type: String,
    pub color: Option<String>,
}

pub struct PetRequest {
    pub name: String,
    pub animal_type: String,
    pub color: Option<String>,
}

pub struct PetResponse {
    pub id: i32,
    pub name: String,
    pub animal_type: String,
    pub color: Option<String>,
}

impl PetResponse {
    pub fn of(pet: Pet) -> PetResponse {
        PetResponse {
            id: pet.id,
            name: pet.name,
            animal_type: pet.animal_type,
            color: pet.color,
        }
    }
}
