use std::path::{ Path, PathBuf };

use piston_window::*;
use opengl_graphics::{GlGraphics, Texture as GlTexture};
use board::Board;
use number_renderer::NumberRenderer;
use settings::Settings;
use ai::{Grid, Direction, get_best_move_expectimax};
use achievements::{Achievements, AchievementType};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    MainMenu,
    Playing,
    Achievements,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PopupAction {
    None,
    Restart,
    BackToMenu,
    Achievement,
}

pub struct App<'a> {
    state: GameState,
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
    settings: &'a Settings,

    logo: Option<GlTexture>,
    comment1: Option<GlTexture>,
    comment2: Option<GlTexture>,
    game_over_text: Option<GlTexture>,
    rules_texture: Option<GlTexture>,
    start_button_texture: Option<GlTexture>,
    quit_button_texture: Option<GlTexture>,
    ai_mode_texture: Option<GlTexture>,
    ai_on_texture: Option<GlTexture>,
    ensure_texture: Option<GlTexture>,
    cancel_texture: Option<GlTexture>,
    restart_texture: Option<GlTexture>,
    menu_texture: Option<GlTexture>,
    undo_texture: Option<GlTexture>,
    ai_button_texture: Option<GlTexture>,
    achievements_button_texture: Option<GlTexture>,
    achievement_unlocked_texture: Option<GlTexture>,
    achievement_points_suffix: Option<GlTexture>,
    achievement_tile_suffix: Option<GlTexture>,
    achievement_completed_texture: Option<GlTexture>,
    back_button_texture: Option<GlTexture>,
    restart_button_texture: Option<GlTexture>,
    window_background_color: [f32; 4],

    ai_mode: bool,
    ai_move_timer: f64,
    
    popup_action: PopupAction,
    achievements: Achievements,
    current_achievement: Option<AchievementType>,
}

fn rgb2rgba(c: [f32; 3]) -> [f32; 4] { [c[0], c[1], c[2], 1.0] }

impl<'a> App<'a> {
    pub fn new(settings: &'a Settings) -> App<'a> {
        App {
            state: GameState::MainMenu,
            board: Board::new(settings),
            number_renderer: None,
            settings: settings,

            logo: None,
            comment1: None,
            comment2: None,
            game_over_text: None,
            rules_texture: None,
            start_button_texture: None,
            quit_button_texture: None,
            ai_mode_texture: None,
            ai_on_texture: None,
            ensure_texture: None,
            cancel_texture: None,
            restart_texture: None,
            menu_texture: None,
            undo_texture: None,
            ai_button_texture: None,
            achievements_button_texture: None,
            achievement_unlocked_texture: None,
            achievement_points_suffix: None,
            achievement_tile_suffix: None,
            achievement_completed_texture: None,
            back_button_texture: None,
            restart_button_texture: None,
            window_background_color: [1.0, 1.0, 1.0, 1.0],

            ai_mode: false,
            ai_move_timer: 0.0,
            
            popup_action: PopupAction::None,
            achievements: Achievements::new(),
            current_achievement: None,
        }
    }

    fn render_ui(&self, c: &Context, gl: &mut GlGraphics) {		
        let logo = self.logo.iter().next().unwrap();
        let (width, height) = logo.get_size();
        let scale = 1.5;
        let y_offset = 20.0;
        let color = self.settings.tiles_colors[3];
        Image::new_color(rgb2rgba(color))
            .rect([self.settings.board_padding, self.settings.board_padding + y_offset, 
                   width as f64 * scale, height as f64 * scale])
            .draw(logo,
                  &DrawState::default(),
                  c.transform,
                  gl);

        Rectangle::new(rgb2rgba(self.settings.label_color))
            .draw(self.settings.best_rect,
                  &DrawState::default(),
                  c.transform,
                  gl);

        Rectangle::new(rgb2rgba(self.settings.label_color))
            .draw(self.settings.score_rect,
                  &DrawState::default(),
                  c.transform,
                  gl);

        let comment1_offset_y = self.settings.comment1_offset_y;
        let comment1 = self.comment1.as_ref().unwrap();
        App::render_comment(self.settings, comment1, comment1_offset_y, c, gl);
        
        let comment2_offset_y = self.settings.comment2_offset_y;
        let comment2 = self.comment2.as_ref().unwrap();
        App::render_comment(self.settings, comment2, comment2_offset_y, c, gl);
    }

    fn render_comment(settings: &Settings, comment: &GlTexture, y: f64, c: &Context, gl: &mut GlGraphics) {
        let (width, height) = comment.get_size();
        let w = settings.window_size[0] as f64 - 2.0 * settings.board_padding;
        let h = height as f64 * w / width as f64;

        Image::new_color(rgb2rgba(settings.text_dark_color))
            .rect([settings.board_padding, y, w, h])
            .draw( comment,
                   &DrawState::default(),
                   c.transform,
                   gl);
    }

    fn render_main_menu(&self, c: &Context, gl: &mut GlGraphics) {
        let center_x = self.settings.window_size[0] as f64 / 2.0;
        let center_y = self.settings.window_size[1] as f64 / 2.0;
        
        if let Some(logo) = self.logo.as_ref() {
            let (width, height) = logo.get_size();
            let scale = 2.0;
            let title_color = self.settings.tiles_colors[5];
            
            Image::new_color(rgb2rgba(title_color))
                .rect([center_x - (width as f64 * scale) / 2.0, 
                    center_y - 200.0, 
                    width as f64 * scale, 
                    height as f64 * scale])
                .draw(logo,
                    &DrawState::default(),
                    c.transform,
                    gl);
        }
        
        if let Some(rules) = self.rules_texture.as_ref() {
            let (width, height) = rules.get_size();
            let scale = 0.6;
            Image::new()
                .rect([center_x - (width as f64 * scale) / 2.0, 
                    center_y - 130.0, 
                    width as f64 * scale, 
                    height as f64 * scale])
                .draw(rules,
                    &DrawState::default(),
                    c.transform,
                    gl);
        }
        
        let btn_width = 200.0;
        let btn_height = 50.0;
        let start_btn_y = center_y + 60.0; 
        Rectangle::new([242.0/255.0, 177.0/255.0, 121.0/255.0, 1.0])
            .draw([center_x - btn_width / 2.0 - 10.0, start_btn_y - 5.0, 
                btn_width + 20.0, btn_height + 10.0],
                &DrawState::default(), c.transform, gl);
        
        if let Some(start_btn) = self.start_button_texture.as_ref() {
            Image::new()
                .rect([center_x - btn_width / 2.0, start_btn_y, btn_width, btn_height])
                .draw(start_btn, &DrawState::default(), c.transform, gl);
        }
        
        let exit_btn_y = start_btn_y + btn_height + 20.0;
        Rectangle::new([242.0/255.0, 177.0/255.0, 121.0/255.0, 1.0])
            .draw([center_x - btn_width / 2.0 - 10.0, exit_btn_y - 5.0, 
                btn_width + 20.0, btn_height + 10.0],
                &DrawState::default(), c.transform, gl);
        
        if let Some(exit_btn) = self.quit_button_texture.as_ref() {
            Image::new()
                .rect([center_x - btn_width / 2.0, exit_btn_y, btn_width, btn_height])
                .draw(exit_btn, &DrawState::default(), c.transform, gl);
        }
        
        let achievements_btn_y = exit_btn_y + btn_height + 20.0;
        Rectangle::new([242.0/255.0, 177.0/255.0, 121.0/255.0, 1.0])
            .draw([center_x - btn_width / 2.0 - 10.0, achievements_btn_y - 5.0, 
                btn_width + 20.0, btn_height + 10.0],
                &DrawState::default(), c.transform, gl);
        
        if let Some(achievements_btn) = self.achievements_button_texture.as_ref() {
            Image::new()
                .rect([center_x - btn_width / 2.0, achievements_btn_y, btn_width, btn_height])
                .draw(achievements_btn, &DrawState::default(), c.transform, gl);
        }
    }

    pub fn load(&mut self) {
        let mut asset_root = PathBuf::new();
        asset_root.push(Path::new(&self.settings.asset_folder));

        let mut logo_path = asset_root.clone();
        logo_path.push(Path::new("logo.png"));
        let mut comment1_path = asset_root.clone();
        comment1_path.push(Path::new("comment1.png"));
        let mut comment2_path = asset_root.clone();
        comment2_path.push(Path::new("comment2.png"));
        let mut game_over_text_path = asset_root.clone();
        game_over_text_path.push(Path::new("game_over.png"));
        let mut rules_path = asset_root.clone();
        rules_path.push(Path::new("rules.png"));
        let mut start_button_path = asset_root.clone();
        start_button_path.push(Path::new("start_button.png"));
        let mut quit_button_path = asset_root.clone();
        quit_button_path.push(Path::new("exit_button.png"));
        let mut ai_mode_path = asset_root.clone();
        ai_mode_path.push(Path::new("ai_mode.png"));
        let mut ai_on_path = asset_root.clone();
        ai_on_path.push(Path::new("ai_on.png"));
        let mut ensure_path = asset_root.clone();
        ensure_path.push(Path::new("ensure.png"));
        let mut cancel_path = asset_root.clone();
        cancel_path.push(Path::new("cancel.png"));
        let mut restart_path = asset_root.clone();
        restart_path.push(Path::new("restart.png"));
        let mut menu_path = asset_root.clone();
        menu_path.push(Path::new("menu.png"));
        let mut undo_path = asset_root.clone();
        undo_path.push(Path::new("undo.png"));
        let mut ai_button_path = asset_root.clone();
        ai_button_path.push(Path::new("ai_button.png"));
        let mut achievements_button_path = asset_root.clone();
        achievements_button_path.push(Path::new("achievements_button.png"));
        let mut achievement_unlocked_path = asset_root.clone();
        achievement_unlocked_path.push(Path::new("achievement_unlocked.png"));
        let mut points_suffix_path = asset_root.clone();
        points_suffix_path.push(Path::new("suffix_points.png"));
        let mut tile_suffix_path = asset_root.clone();
        tile_suffix_path.push(Path::new("suffix_tile.png"));
        let mut completed_path = asset_root.clone();
        completed_path.push(Path::new("completed.png"));
        let mut back_button_path = asset_root.clone();
        back_button_path.push(Path::new("back_button.png"));
        let mut restart_button_path = asset_root.clone();
        restart_button_path.push(Path::new("restart_button.png"));

        self.number_renderer = Some(NumberRenderer::new());
        let texture_settings = TextureSettings::new();
        self.logo = Some(GlTexture::from_path(&logo_path, &texture_settings).unwrap());
        self.comment1 = Some(GlTexture::from_path(&comment1_path, &texture_settings).unwrap());
        self.comment2 = Some(GlTexture::from_path(&comment2_path, &texture_settings).unwrap());
        self.game_over_text = Some(GlTexture::from_path(&game_over_text_path, &texture_settings).unwrap());
        self.rules_texture = Some(GlTexture::from_path(&rules_path, &texture_settings).unwrap());
        self.start_button_texture = Some(GlTexture::from_path(&start_button_path, &texture_settings).unwrap());
        self.quit_button_texture = Some(GlTexture::from_path(&quit_button_path, &texture_settings).unwrap());
        self.ai_mode_texture = Some(GlTexture::from_path(&ai_mode_path, &texture_settings).unwrap());
        self.ai_on_texture = Some(GlTexture::from_path(&ai_on_path, &texture_settings).unwrap());
        self.ensure_texture = Some(GlTexture::from_path(&ensure_path, &texture_settings).unwrap());
        self.cancel_texture = Some(GlTexture::from_path(&cancel_path, &texture_settings).unwrap());
        self.restart_texture = Some(GlTexture::from_path(&restart_path, &texture_settings).unwrap());
        self.menu_texture = Some(GlTexture::from_path(&menu_path, &texture_settings).unwrap());
        self.undo_texture = Some(GlTexture::from_path(&undo_path, &texture_settings).unwrap());
        self.ai_button_texture = Some(GlTexture::from_path(&ai_button_path, &texture_settings).unwrap());
        self.achievements_button_texture = Some(GlTexture::from_path(&achievements_button_path, &texture_settings).unwrap());
        self.achievement_unlocked_texture = Some(GlTexture::from_path(&achievement_unlocked_path, &texture_settings).unwrap());
        self.achievement_points_suffix = Some(GlTexture::from_path(&points_suffix_path, &texture_settings).unwrap());
        self.achievement_tile_suffix = Some(GlTexture::from_path(&tile_suffix_path, &texture_settings).unwrap());
        self.achievement_completed_texture = Some(GlTexture::from_path(&completed_path, &texture_settings).unwrap());
        self.back_button_texture = Some(GlTexture::from_path(&back_button_path, &texture_settings).unwrap());
        self.restart_button_texture = Some(GlTexture::from_path(&restart_button_path, &texture_settings).unwrap());
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let area = args.window_size;
        let ref c = Context::new_abs(area[0], area[1]);

        let w_bg_col = if self.state == GameState::Playing {
            self.get_background_color()
        } else {
            self.window_background_color
        };
        
        gl.draw(args.viewport(), |_, gl| {
            clear(w_bg_col, gl);
            
            match self.state {
                GameState::MainMenu => {
                    self.render_main_menu(c, gl);
                }
                GameState::Achievements => {
                    self.render_achievements_page(c, gl);
                }
                GameState::Playing => {
                    let is_game_over = self.board.is_game_over();
                    self.render_ui(c, gl);
                    self.board.render(self.number_renderer.iter().next().unwrap(), c, gl);
                    
                    self.render_ai_indicator(c, gl);
                    self.render_ai_hint(c, gl);
                    self.render_undo_button(c, gl);
                    self.render_restart_button(c, gl);
                    self.render_back_button(c, gl);
                    
                    if is_game_over {
                        self.render_game_over(c, gl);
                    }
                    
                    self.render_popup(c, gl);
                }
            }
        });
    }

    fn render_ai_indicator(&self, c: &Context, gl: &mut GlGraphics) {
        let rect = self.settings.ai_button_rect;
        let color = if self.ai_mode { 
            [0.0, 0.8, 0.0, 1.0] 
        } else { 
            rgb2rgba(self.settings.label_color)
        };
        
        Rectangle::new(color)
            .draw(rect,
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        if let Some(ref ai_button_tex) = self.ai_button_texture {
            let (width, height) = ai_button_tex.get_size();
            let w = rect[2];
            let h = height as f64 * w / width as f64;
            
            let image_color = if self.ai_mode {
                rgb2rgba(self.settings.text_light_color)
            } else {
                rgb2rgba(self.settings.text_light_color)
            };
            
            Image::new_color(image_color)
                .rect([rect[0], rect[1] + (rect[3] - h) / 2.0, w, h])
                .draw(ai_button_tex,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }
    
    fn render_ai_hint(&self, c: &Context, gl: &mut GlGraphics) {
        if let Some(ref ai_mode_tex) = self.ai_mode_texture {
            let hint_x = self.settings.board_padding;
            let hint_y = self.settings.comment2_offset_y + 25.0;
            let (width, height) = ai_mode_tex.get_size();
            let scale = 0.8;
            
            Image::new()
                .rect([hint_x, hint_y, 
                       width as f64 * scale, height as f64 * scale])
                .draw(ai_mode_tex,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }
    
    fn render_undo_button(&self, c: &Context, gl: &mut GlGraphics) {
        let rect = self.settings.undo_rect;
        let color = if self.board.can_undo() {
            rgb2rgba(self.settings.label_color)
        } else {
            [0.5, 0.5, 0.5, 1.0]
        };
        
        Rectangle::new(color)
            .draw(rect,
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        if let Some(ref undo_tex) = self.undo_texture {
            let (width, height) = undo_tex.get_size();
            let w = rect[2];
            let h = height as f64 * w / width as f64;
            
            let image_color = if self.board.can_undo() {
                rgb2rgba(self.settings.text_light_color)
            } else {
                [0.3, 0.3, 0.3, 1.0]
            };
            
            Image::new_color(image_color)
                .rect([rect[0], rect[1] + (rect[3] - h) / 2.0, w, h])
                .draw(undo_tex,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }
    
    fn render_restart_button(&self, c: &Context, gl: &mut GlGraphics) {
        let rect = self.settings.restart_button_rect;
        
        Rectangle::new(rgb2rgba(self.settings.label_color))
            .draw(rect,
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        if let Some(ref restart_tex) = self.restart_button_texture {
            let (width, height) = restart_tex.get_size();
            let w = rect[2];
            let h = height as f64 * w / width as f64;
            
            Image::new_color(rgb2rgba(self.settings.text_light_color))
                .rect([rect[0], rect[1] + (rect[3] - h) / 2.0, w, h])
                .draw(restart_tex,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }
    
    fn render_back_button(&self, c: &Context, gl: &mut GlGraphics) {
        let rect = self.settings.back_button_rect;
        
        Rectangle::new(rgb2rgba(self.settings.label_color))
            .draw(rect,
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        if let Some(ref back_tex) = self.back_button_texture {
            let (width, height) = back_tex.get_size();
            let w = rect[2];
            let h = height as f64 * w / width as f64;
            
            Image::new_color(rgb2rgba(self.settings.text_light_color))
                .rect([rect[0], rect[1] + (rect[3] - h) / 2.0, w, h])
                .draw(back_tex,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }
    
    fn get_background_color(&self) -> [f32; 4] {
        let score = self.board.score;
        let colors = &self.settings.score_background_colors;
        
        let color_idx = if score < 1000 {
            0
        } else if score < 3000 {
            1
        } else if score < 5000 {
            2
        } else if score < 7000 {
            3
        } else if score < 10000 {
            4
        } else if score < 15000 {
            5
        } else {
            6
        };
        
        if let Some(color) = colors.get(color_idx) {
            [color[0], color[1], color[2], 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    }
    
    fn render_popup(&self, c: &Context, gl: &mut GlGraphics) {
        if self.popup_action == PopupAction::None {
            return;
        }
        
        let overlay_color = [0.0, 0.0, 0.0, 0.6];
        Rectangle::new(overlay_color)
            .draw([0.0, 0.0,
                   self.settings.window_size[0] as f64,
                   self.settings.window_size[1] as f64],
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        let center_x = self.settings.window_size[0] as f64 / 2.0;
        let center_y = self.settings.window_size[1] as f64 / 2.0;
        
        let bg_color = [238.0/255.0, 228.0/255.0, 218.0/255.0, 1.0];
        let border_color = [187.0/255.0, 173.0/255.0, 160.0/255.0, 1.0];
        
        if self.popup_action == PopupAction::Achievement {
            Rectangle::new(border_color)
                .draw([center_x - 160.0, center_y - 80.0, 320.0, 180.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            Rectangle::new(bg_color)
                .draw([center_x - 155.0, center_y - 75.0, 310.0, 170.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            if let Some(ref tex) = self.achievement_unlocked_texture {
                let (width, height) = tex.get_size();
                let w = 200.0;
                let h = height as f64 * w / width as f64;
                Image::new_color([0.0, 0.8, 0.0, 1.0])
                    .rect([center_x - w / 2.0, center_y - 40.0, w, h])
                    .draw(tex, &DrawState::default(), c.transform, gl);
            }
            
            if let Some(achievement) = self.current_achievement {
                if let Some(ref number_renderer) = self.number_renderer {
                    let text_y = center_y + 10.0;
                    let color = rgb2rgba(self.settings.text_dark_color);
                    
                    let number_width = number_renderer.render(
                        achievement.get_value() as u32,
                        center_x,
                        text_y,
                        80.0,
                        [color[0], color[1], color[2]],
                        c,
                        gl);
                    
                    if achievement.is_score() {
                        if let Some(ref tex) = self.achievement_points_suffix {
                            let (width, height) = tex.get_size();
                            let w = 40.0;
                            let h = height as f64 * w / width as f64;
                            let suffix_x = center_x + number_width / 2.0 + 5.0;
                            Image::new_color(color)
                                .rect([suffix_x, text_y - h / 2.0, w, h])
                                .draw(tex, &DrawState::default(), c.transform, gl);
                        }
                    } else {
                        if let Some(ref tex) = self.achievement_tile_suffix {
                            let (width, height) = tex.get_size();
                            let w = 60.0;
                            let h = height as f64 * w / width as f64;
                            let suffix_x = center_x + number_width / 2.0 + 5.0;
                            Image::new_color(color)
                                .rect([suffix_x, text_y - h / 2.0, w, h])
                                .draw(tex, &DrawState::default(), c.transform, gl);
                        }
                    }
                }
            }
            
            let border_color = [0.5, 0.5, 0.5, 0.5];
            let button_w = 60.0;
            let button_h = 25.0;
            let button_y = center_y + 60.0;
            
            Rectangle::new(border_color)
                .draw([center_x - 35.0, button_y - 2.5, button_w + 10.0, button_h + 5.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            if let Some(ref ensure_tex) = self.ensure_texture {
                let (width, height) = ensure_tex.get_size();
                let w = button_w;
                let h = height as f64 * w / width as f64;
                Image::new_color(rgb2rgba(self.settings.text_dark_color))
                    .rect([center_x - 30.0, button_y, w, h.min(button_h)])
                    .draw(ensure_tex, &DrawState::default(), c.transform, gl);
            }
        } else {
            Rectangle::new(border_color)
                .draw([center_x - 160.0, center_y - 80.0, 320.0, 160.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            Rectangle::new(bg_color)
                .draw([center_x - 155.0, center_y - 75.0, 310.0, 150.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            let title_texture = match self.popup_action {
                PopupAction::Restart => &self.restart_texture,
                PopupAction::BackToMenu => &self.menu_texture,
                _ => &self.restart_texture,
            };
            
            if let Some(ref tex) = *title_texture {
                let (width, height) = tex.get_size();
                let w = 200.0;
                let h = height as f64 * w / width as f64;
                Image::new_color(rgb2rgba(self.settings.text_dark_color))
                    .rect([center_x - w / 2.0, center_y - 25.0, w, h])
                    .draw(tex, &DrawState::default(), c.transform, gl);
            }
            
            let border_color = [0.5, 0.5, 0.5, 0.5];
            let button_w = 60.0;
            let button_h = 25.0;
            let button_y = center_y + 35.0;
            
            Rectangle::new(border_color)
                .draw([center_x - 105.0, button_y - 2.5, button_w + 10.0, button_h + 5.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            if let Some(ref ensure_tex) = self.ensure_texture {
                let (width, height) = ensure_tex.get_size();
                let w = button_w;
                let h = height as f64 * w / width as f64;
                Image::new_color(rgb2rgba(self.settings.text_dark_color))
                    .rect([center_x - 100.0, button_y, w, h.min(button_h)])
                    .draw(ensure_tex, &DrawState::default(), c.transform, gl);
            }
            
            Rectangle::new(border_color)
                .draw([center_x + 35.0, button_y - 2.5, button_w + 10.0, button_h + 5.0],
                      &DrawState::default(),
                      c.transform,
                      gl);
            
            if let Some(ref cancel_tex) = self.cancel_texture {
                let (width, height) = cancel_tex.get_size();
                let w = button_w;
                let h = height as f64 * w / width as f64;
                Image::new_color(rgb2rgba(self.settings.text_dark_color))
                    .rect([center_x + 40.0, button_y, w, h.min(button_h)])
                    .draw(cancel_tex, &DrawState::default(), c.transform, gl);
            }
        }
    }
    
    fn render_achievements_page(&self, c: &Context, gl: &mut GlGraphics) {
        let center_x = self.settings.window_size[0] as f64 / 2.0;
        
        let back_btn_y = 20.0;
        Rectangle::new([242.0/255.0, 177.0/255.0, 121.0/255.0, 1.0])
            .draw([20.0, back_btn_y, 100.0, 40.0],
                  &DrawState::default(), c.transform, gl);
        
        if let Some(ref back_tex) = self.back_button_texture {
            let (width, height) = back_tex.get_size();
            let w = 80.0;
            let h = height as f64 * w / width as f64;
            Image::new_color(rgb2rgba(self.settings.text_dark_color))
                .rect([30.0, back_btn_y + (40.0 - h) / 2.0, w, h])
                .draw(back_tex, &DrawState::default(), c.transform, gl);
        }
        
        let progress = self.achievements.get_unlocked_count() as f64 / 
                       Achievements::get_total_count() as f64 * 100.0;
        
        let progress_bar_y = 80.0;
        Rectangle::new([187.0/255.0, 173.0/255.0, 160.0/255.0, 1.0])
            .draw([center_x - 150.0, progress_bar_y, 300.0, 30.0],
                  &DrawState::default(), c.transform, gl);
        
        Rectangle::new([0.0, 0.8, 0.0, 1.0])
            .draw([center_x - 145.0, progress_bar_y + 5.0, 
                   (300.0 - 10.0) * progress / 100.0, 20.0],
                  &DrawState::default(), c.transform, gl);
        
        let tile_size = 110.0;
        let tile_padding = 15.0;
        let cols = 3;
        let rows = 4;
        
        let grid_width = cols as f64 * tile_size + (cols - 1) as f64 * tile_padding;
        let grid_start_x = center_x - grid_width / 2.0;
        let grid_start_y = 130.0;
        
        let all_achievements = Achievements::get_all_achievements();
        
        for (idx, &achievement) in all_achievements.iter().enumerate() {
            let col = idx % cols;
            let row = idx / cols;
            
            let tile_x = grid_start_x + col as f64 * (tile_size + tile_padding);
            let tile_y = grid_start_y + row as f64 * (tile_size + tile_padding);
            
            let is_unlocked = self.achievements.is_unlocked(achievement);
            
            let tile_color = if is_unlocked {
                [237.0/255.0, 200.0/255.0, 80.0/255.0, 1.0]
            } else {
                [200.0/255.0, 190.0/255.0, 180.0/255.0, 1.0]
            };
            
            Rectangle::new(tile_color)
                .draw([tile_x, tile_y, tile_size, tile_size],
                      &DrawState::default(), c.transform, gl);
            
            let text_color = if is_unlocked {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                rgb2rgba(self.settings.text_dark_color)
            };
            
            if let Some(ref number_renderer) = self.number_renderer {
                let text_y = tile_y + tile_size / 2.0 - 10.0;
                let color = text_color;
                
                let _ = number_renderer.render(
                    achievement.get_value() as u32,
                    tile_x + tile_size / 2.0,
                    text_y,
                    tile_size - 20.0,
                    [color[0], color[1], color[2]],
                    c,
                    gl);
                
                let suffix_y = tile_y + tile_size / 2.0 + 20.0;
                
                if achievement.is_score() {
                    if let Some(ref tex) = self.achievement_points_suffix {
                        let (width, height) = tex.get_size();
                        let w = 40.0;
                        let h = height as f64 * w / width as f64;
                        Image::new_color(color)
                            .rect([tile_x + tile_size / 2.0 - w / 2.0, suffix_y - h / 2.0, w, h])
                            .draw(tex, &DrawState::default(), c.transform, gl);
                    }
                } else {
                    if let Some(ref tex) = self.achievement_tile_suffix {
                        let (width, height) = tex.get_size();
                        let w = 60.0;
                        let h = height as f64 * w / width as f64;
                        Image::new_color(color)
                            .rect([tile_x + tile_size / 2.0 - w / 2.0, suffix_y - h / 2.0, w, h])
                            .draw(tex, &DrawState::default(), c.transform, gl);
                    }
                }
            }
            
            if is_unlocked {
                if let Some(ref tex) = self.achievement_completed_texture {
                    let (width, height) = tex.get_size();
                    let w = 80.0;
                    let h = height as f64 * w / width as f64;
                    Image::new_color([0.0, 0.8, 0.0, 1.0])
                        .rect([tile_x + tile_size / 2.0 - w / 2.0, tile_y + tile_size - 25.0, w, h])
                        .draw(tex, &DrawState::default(), c.transform, gl);
                }
            }
        }
    }

    fn render_game_over(&self, c: &Context, gl: &mut GlGraphics) {
        let overlay_color = [0.0, 0.0, 0.0, 0.5];
        
        Rectangle::new(overlay_color)
            .draw([0.0, 0.0,
                   self.settings.window_size[0] as f64,
                   self.settings.window_size[1] as f64],
                  &DrawState::default(),
                  c.transform,
                  gl);
        
        let center_x = self.settings.window_size[0] as f64 / 2.0;
        let center_y = self.settings.window_size[1] as f64 / 2.0;
        
        if let Some(game_over_img) = self.game_over_text.as_ref() {
            Image::new()
                .rect([center_x - 150.0, center_y - 80.0, 300.0, 160.0])
                .draw(game_over_img,
                      &DrawState::default(),
                      c.transform,
                      gl);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.board.update(args.dt);
        
        if self.state == GameState::Playing && self.popup_action == PopupAction::None {
            self.check_achievements();
            
            if self.ai_mode && !self.board.is_game_over() {
                self.ai_move_timer += args.dt;
                
                if self.ai_move_timer >= 0.5 && !self.board.is_locking() {
                    self.ai_move_timer = 0.0;
                    self.perform_ai_move();
                }
            }
        }
    }
    
    fn check_achievements(&mut self) {
        if self.board.has_new_tile_values() {
            let new_values = self.board.take_new_tile_values();
            for &value in &new_values {
                self.achievements.check_tile_achievement(value);
            }
        }
        
        self.achievements.check_score_achievement(self.board.score);
        
        if self.achievements.has_new_achievements() && self.popup_action == PopupAction::None {
            let new_achievements = self.achievements.take_new_achievements();
            if let Some(&first) = new_achievements.first() {
                self.current_achievement = Some(first);
                self.popup_action = PopupAction::Achievement;
            }
        }
    }

    fn perform_ai_move(&mut self) {
        if self.board.is_locking() {
            return;
        }
        
        let tiles_info = self.board.get_tiles_info();
        let grid = Grid::from_tiles(&tiles_info);
        
        if let Some(dir) = get_best_move_expectimax(&grid, 2) {
            match dir {
                Direction::Left => self.board.merge_from_right_to_left(),
                Direction::Right => self.board.merge_from_left_to_right(),
                Direction::Up => self.board.merge_from_bottom_to_top(),
                Direction::Down => self.board.merge_from_top_to_bottom(),
            }
            
            if self.board.is_locking() {
                self.ai_move_timer = 0.0;
                return;
            }
        }
        
        self.try_fallback_moves();
    }
    
    fn try_fallback_moves(&mut self) {
        if self.board.is_locking() {
            return;
        }
        
        self.board.merge_from_right_to_left();
        if self.board.is_locking() {
            self.ai_move_timer = 0.0;
            return;
        }
        
        self.board.merge_from_left_to_right();
        if self.board.is_locking() {
            self.ai_move_timer = 0.0;
            return;
        }
        
        self.board.merge_from_bottom_to_top();
        if self.board.is_locking() {
            self.ai_move_timer = 0.0;
            return;
        }
        
        self.board.merge_from_top_to_bottom();
        if self.board.is_locking() {
            self.ai_move_timer = 0.0;
        }
    }

    pub fn is_main_menu(&self) -> bool {
        self.state == GameState::MainMenu
    }

    pub fn key_press(&mut self, args: &Button) {
		use piston_window::Button::Keyboard;
		
        if self.state == GameState::MainMenu {
            if *args == Keyboard(Key::Space) {
                self.state = GameState::Playing;
                self.board = Board::new(self.settings);
            }
            if *args == Keyboard(Key::Escape) {
                std::process::exit(0);
            }
            return;
        }
        
        if self.state == GameState::Achievements {
            if *args == Keyboard(Key::Escape) || *args == Keyboard(Key::Return) {
                self.state = GameState::MainMenu;
            }
            return;
        }
        
        if self.popup_action != PopupAction::None {
            if *args == Keyboard(Key::Return) {
                match self.popup_action {
                    PopupAction::Restart => {
                        self.board = Board::new(self.settings);
                        self.ai_mode = false;
                    }
                    PopupAction::BackToMenu => {
                        self.state = GameState::MainMenu;
                        self.ai_mode = false;
                    }
                    _ => {}
                }
                self.popup_action = PopupAction::None;
            } else if *args == Keyboard(Key::Escape) {
                self.popup_action = PopupAction::None;
            }
            return;
        }
        
        if *args == Keyboard(Key::LShift) || *args == Keyboard(Key::RShift) {
            self.ai_mode = !self.ai_mode;
            return;
        }
        
        if *args == Keyboard(Key::Space) {
            self.popup_action = PopupAction::Restart;
            return;
        }
        
        if *args == Keyboard(Key::Return) {
            self.popup_action = PopupAction::BackToMenu;
            return;
        }
        
        if *args == Keyboard(Key::Z) {
            if !self.ai_mode && !self.board.is_locking() && self.board.can_undo() {
                self.board.undo();
            }
            return;
        }
        
        if self.ai_mode {
            return;
        }
        
        if *args == Keyboard(Key::Left) {
            self.board.merge_from_right_to_left();
        }

        if *args == Keyboard(Key::Right) {
            self.board.merge_from_left_to_right();
        }

        if *args == Keyboard(Key::Up) {
            self.board.merge_from_bottom_to_top();
        }

        if *args == Keyboard(Key::Down) {
            self.board.merge_from_top_to_bottom();
        }
    }

    pub fn mouse_press(&mut self, args: &Button, mouse_pos: [f64; 2]) {
        use piston_window::Button::Mouse;
        
        if *args == Mouse(MouseButton::Left) {
            if self.popup_action != PopupAction::None {
                let center_x = self.settings.window_size[0] as f64 / 2.0;
                let center_y = self.settings.window_size[1] as f64 / 2.0;
                let button_h = 25.0;
                let button_w = 60.0;
                
                if self.popup_action == PopupAction::Achievement {
                    let button_y = center_y + 60.0;
                    let confirm_x1 = center_x - 30.0;
                    let confirm_y1 = button_y;
                    let confirm_x2 = confirm_x1 + button_w;
                    let confirm_y2 = confirm_y1 + button_h;
                    
                    if mouse_pos[0] >= confirm_x1 && mouse_pos[0] <= confirm_x2 &&
                       mouse_pos[1] >= confirm_y1 && mouse_pos[1] <= confirm_y2 {
                        self.popup_action = PopupAction::None;
                        self.current_achievement = None;
                        return;
                    }
                } else {
                    let button_y = center_y + 35.0;
                    
                    let confirm_x1 = center_x - 100.0;
                    let confirm_y1 = button_y;
                    let confirm_x2 = confirm_x1 + button_w;
                    let confirm_y2 = confirm_y1 + button_h;
                    
                    if mouse_pos[0] >= confirm_x1 && mouse_pos[0] <= confirm_x2 &&
                       mouse_pos[1] >= confirm_y1 && mouse_pos[1] <= confirm_y2 {
                        match self.popup_action {
                            PopupAction::Restart => {
                                self.board = Board::new(self.settings);
                                self.ai_mode = false;
                            }
                            PopupAction::BackToMenu => {
                                self.state = GameState::MainMenu;
                                self.ai_mode = false;
                            }
                            _ => {}
                        }
                        self.popup_action = PopupAction::None;
                        return;
                    }
                    
                    let cancel_x1 = center_x + 40.0;
                    let cancel_y1 = button_y;
                    let cancel_x2 = cancel_x1 + button_w;
                    let cancel_y2 = cancel_y1 + button_h;
                    
                    if mouse_pos[0] >= cancel_x1 && mouse_pos[0] <= cancel_x2 &&
                       mouse_pos[1] >= cancel_y1 && mouse_pos[1] <= cancel_y2 {
                        self.popup_action = PopupAction::None;
                        return;
                    }
                }
            }
            
            if self.state == GameState::Playing {
                let ai_rect = self.settings.ai_button_rect;
                if mouse_pos[0] >= ai_rect[0] && mouse_pos[0] <= ai_rect[0] + ai_rect[2] &&
                   mouse_pos[1] >= ai_rect[1] && mouse_pos[1] <= ai_rect[1] + ai_rect[3] {
                    self.ai_mode = !self.ai_mode;
                    return;
                }
                
                let undo_rect = self.settings.undo_rect;
                if mouse_pos[0] >= undo_rect[0] && mouse_pos[0] <= undo_rect[0] + undo_rect[2] &&
                   mouse_pos[1] >= undo_rect[1] && mouse_pos[1] <= undo_rect[1] + undo_rect[3] {
                    if !self.ai_mode && !self.board.is_locking() && self.board.can_undo() {
                        self.board.undo();
                    }
                    return;
                }
                
                let restart_rect = self.settings.restart_button_rect;
                if mouse_pos[0] >= restart_rect[0] && mouse_pos[0] <= restart_rect[0] + restart_rect[2] &&
                   mouse_pos[1] >= restart_rect[1] && mouse_pos[1] <= restart_rect[1] + restart_rect[3] {
                    if !self.board.is_locking() {
                        self.popup_action = PopupAction::Restart;
                    }
                    return;
                }
                
                let back_rect = self.settings.back_button_rect;
                if mouse_pos[0] >= back_rect[0] && mouse_pos[0] <= back_rect[0] + back_rect[2] &&
                   mouse_pos[1] >= back_rect[1] && mouse_pos[1] <= back_rect[1] + back_rect[3] {
                    self.popup_action = PopupAction::BackToMenu;
                    return;
                }
                
                return;
            }
            
            let center_x = self.settings.window_size[0] as f64 / 2.0;
            let center_y = self.settings.window_size[1] as f64 / 2.0;
            
            let btn_width = 200.0;
            let btn_height = 50.0;
            
            let start_btn_y = center_y + 60.0;
            if mouse_pos[0] >= center_x - btn_width / 2.0 - 10.0 && 
            mouse_pos[0] <= center_x + btn_width / 2.0 + 10.0 &&
            mouse_pos[1] >= start_btn_y - 5.0 && 
            mouse_pos[1] <= start_btn_y + btn_height + 5.0 {
                self.state = GameState::Playing;
                self.board = Board::new(self.settings);
                return;
            }
            
            let exit_btn_y = start_btn_y + btn_height + 20.0;
            if mouse_pos[0] >= center_x - btn_width / 2.0 - 10.0 && 
            mouse_pos[0] <= center_x + btn_width / 2.0 + 10.0 &&
            mouse_pos[1] >= exit_btn_y - 5.0 && 
            mouse_pos[1] <= exit_btn_y + btn_height + 5.0 {
                std::process::exit(0);
            }
            
            let achievements_btn_y = exit_btn_y + btn_height + 20.0;
            if mouse_pos[0] >= center_x - btn_width / 2.0 - 10.0 && 
            mouse_pos[0] <= center_x + btn_width / 2.0 + 10.0 &&
            mouse_pos[1] >= achievements_btn_y - 5.0 && 
            mouse_pos[1] <= achievements_btn_y + btn_height + 5.0 {
                self.state = GameState::Achievements;
            }
        }
        
        if self.state == GameState::Achievements {
            if mouse_pos[0] >= 20.0 && mouse_pos[0] <= 120.0 &&
               mouse_pos[1] >= 20.0 && mouse_pos[1] <= 60.0 {
                self.state = GameState::MainMenu;
            }
        }
    }
}