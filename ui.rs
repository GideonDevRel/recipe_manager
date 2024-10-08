use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};
use iced::{Element, Length, Sandbox, Settings};
use iced::theme;
use iced::Color;
use crate::manager::RecipeManager;
use crate::recipe::Recipe;

pub struct RecipeManagerGUI {
    recipe_manager: RecipeManager,
    recipe_name: String,
    recipe_ingridients: String,
    recipe_instructions: String,
    recipe_serivngs: String,
    selected_recipe: Option<Recipe>,
    error_message: Option<String>,
    editing: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddReceipe,
    EditRecipe(u32),
    UpdateRecipe,
    CancelEdit,
    RecipeNameChanged(String),
    RecipeIngridientsChanged(String),
    RecipeInstructionsChanged(String),
    RecipeServingChanged(String),
    RecipeSelected(Recipe),
    SaveRecipes,
    LoadRecipes,
}

impl Sandbox for RecipeManagerGUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            recipe_manager: RecipeManager::new(),
            recipe_name: String::new(),
            recipe_ingridients: String::new(),
            recipe_instructions: String::new(),
            recipe_serivngs: String::new(),
            selected_recipe: None,
            error_message: None,
            editing: false,
        }
    }
    fn title(&self) -> String {
        String::from("Recipe Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddReceipe => {
                if !self.recipe_name.is_empty() {
                    let servings = self.recipe_serivngs.parse().unwrap_or(1);
                    self.recipe_manager.add_recipe(
                        self.recipe_name.clone(),
                        self.recipe_ingridients.split(',').map(String::from).collect(),
                        self.recipe_instructions.split('n').map(String::from).collect(),
                        servings,
                    );
                    self.recipe_name.clear();
                    self.recipe_ingridients.clear();
                    self.recipe_instructions.clear();
                    self.recipe_serivngs.clear();
                }
            }
            Message::EditRecipe(id) => {
                if let Some(recipe) = self.recipe_manager.get_recipe(id) {
                    self.recipe_name = recipe_name.clone();
                    self.recipe_ingridients = recipe_ingridients.join(" ");
                    self.recipe_instructions = recipe_instructions.join("n");
                    self.recipe_serivngs = recipe_serivngs.to_string();
                    self.selected_recipe = Some(recipe.clone());
                    self.editing = true;
                }
            }
            Message::UpdateRecipe => {
                if let Some(recipe) = &self.selected_recipe {
                    let servings = self.recipe_serivngs.parse().unwrap_or(recipe_serivngs);
                    self.recipe_manager.update_recipe(
                        recipe.id,
                        self.recipe_name.clone(),
                        self.recipe_ingridients.split(',').map(String::from).collect(),
                        self.recipe_instructions.split('n').map(String::from).collect(),
                        servings,
                    );
                    self.editing = false;
                    self.selected_recipe = None; 
                }
            }
            Message::CancelEdit => {
                self.editing = false;
                self.recipe_name.clear();
                self.recipe_ingridients.clear();
                self.recipe_instructions.clear();
                self.recipe_serivngs.clear();
            }
            Message::RecipeNameChanged(name) => {
                self.recipe_name = name;
            }
            Message::RecipeIngridientsChanged(ingridients) => {
                self.recipe_ingridients = ingridients;
            }
            Message::RecipeInstructionsChanged(instructions) => {
                self.recipe_instructions = instructions;
            }
            Message::RecipeServingChanged(servings) => {
                self.recipe_serivngs = servings;
            }
            Message::RecipeSelected(recipe) => {
                self.selected_recipe = Some(recipe);
                self.editing = false;
            }
            Message::DeleteRecipe(id) => {
                if self.recipe_manager.delete_recipe(id) {
                    self.selected_recipe = None;
                }
            }
            Message::SaveRecipes => {
                if let Err(e) = self.recipe_manager.save_to_file("recipes.json") {
                    self.error_message = Some(format!("failed to save recipes. {}", e));
                }
            }
            Message::LoadRecipes => {
                match self.recipe_manager.load_from_file("recipe.json") {
                    Ok(_) => self.selected_recipe = None,
                    Err(e) => self.error_message = Some(format!("Failed to load recipes: {}", e))
                }
            }
        }
    }

    
}