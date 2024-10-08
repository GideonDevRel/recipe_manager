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
    updateRecipe,
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
            selected_recipe: none,
            error_message: none,
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
                    
                }
            }
        }
    }
}