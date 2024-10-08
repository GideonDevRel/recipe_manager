use crate::recipe::Recipe;
use std::fs;

pub struct RecipeManager {
    recipes: Vec<Recipe>,
    next_id: u32,
}

impl RecipeManager {
    pub fn new() -> Self {
        RecipeManager{
            recipes: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_recipe(&mut self, name: String, ingridients:Vec<String>, instructions: Vec<String>, servings: u32) -> u32 {
        let id = self.next_id;
        self.recipes.push(Recipe::new(id, name, ingridients, instructions, servings));
        self.next_id += 1;
        id
    }

    pub fn get_all_recipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }

    pub fn get_recipe(&self, id: u32) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    pub fn update_recipe(&mut self, id: u32, name: String, ingridients: Vec<String>, instructions: Vec<String>, servings: u32) -> bool {
        if let Some(recipe) = self.recipes.iter_mut().find(|r| r.id == id) {
            recipe.name = name;
            recipe.ingridients = ingridients;
            recipe.instructions = instructions;
            recipe.servings = servings;
            true
        } else {
            false
        }
    }

    pub fn delete_recipe(&mut self, id: u32) -> bool {
        let initial_len = self.recipes.len();
        self.recipes.retain(|r| r.id != id);
        self.recipes.len() < initial_len
    }

    pub fn save_to_file(&mut self, filename: &str) -> std::io::Result<()> {
        let json = fs::read_to_string(&self.recipes)?;
        fs::write(filename, json)
    }

    pub fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let json = fs::read_to_string(filename)?;
        self.recipes = serde_json::from_str(&json)?;
        self.next_id = self.recipes.iter().map(|r| r.id).max().unwrap_or(0) + 1;
        ok(())
    }
}