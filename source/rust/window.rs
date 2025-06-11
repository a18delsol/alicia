/*
* Copyright (c) 2025 a18delsol
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::{base, script::Script, status::*};

//================================================================

use crate::base::helper::*;

//================================================================

// window structure, responsible for drawing the missing/failure interface.
pub struct Window {
    font: Font,
    logo: Texture2D,
    point: Vector2,
    count: i32,
}

impl Window {
    const COLOR_PRIMARY_MAIN: Color = Color {
        r: 156,
        g: 39,
        b: 176,
        a: 255,
    };
    const COLOR_TEXT_WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    //================================================================

    const GRADIENT_POINT_Y: f32 = 4.0;
    const GRADIENT_SHAPE_Y: i32 = 6;
    const GRADIENT_COLOR_MAX: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 99,
    };
    const GRADIENT_COLOR_MIN: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    //================================================================

    const LOGO_SHAPE: f32 = 160.0;

    //================================================================

    const CARD_ROUND_SHAPE: f32 = 0.25;
    const CARD_ROUND_COUNT: i32 = 4;

    //================================================================

    const TEXT_SHAPE: f32 = 24.0;
    const TEXT_SPACE: f32 = 1.0;

    //================================================================

    const BUTTON_SHAPE: Vector2 = Vector2 { x: 160.0, y: 32.0 };
    const BUTTON_TEXT_SHIFT: Vector2 = Vector2 { x: 8.0, y: 4.0 };
    const BUTTON_SHIFT: f32 = 8.0;

    //================================================================

    // get a new window instance.
    pub fn new() -> Self {
        // load font.
        let font = unsafe {
            LoadFontFromMemory(
                Script::rust_to_c_string(".ttf").unwrap().as_ptr(),
                Status::FONT.as_ptr(),
                Status::FONT.len() as i32,
                Self::TEXT_SHAPE as i32,
                core::ptr::null_mut(),
                0,
            )
        };

        // load logo.
        let logo = unsafe {
            let image = LoadImageFromMemory(
                Script::rust_to_c_string(".png").unwrap().as_ptr(),
                Status::LOGO.as_ptr(),
                Status::LOGO.len() as i32,
            );

            LoadTextureFromImage(image)
        };

        Self {
            font,
            logo,
            point: Vector2 { x: 0.0, y: 0.0 },
            count: i32::default(),
        }
    }

    // draw missing window layout.
    pub async fn missing(&mut self) -> Option<Status> {
        while unsafe { !WindowShouldClose() } {
            let draw_shape = Vector2 {
                x: unsafe { GetScreenWidth() as f32 },
                y: unsafe { GetScreenHeight() as f32 },
            };
            let logo_shape = Vector2 {
                x: self.logo.width as f32,
                y: self.logo.height as f32,
            };
            let logo_point = Vector2 {
                x: (draw_shape.x * 0.5) - (logo_shape.x * 0.5),
                y: (draw_shape.y * 0.5) - (logo_shape.y * 0.5) - (Self::LOGO_SHAPE * 0.5),
            };
            let card_shape = Rectangle {
                x: 0.0,
                y: 0.0,
                width: draw_shape.x,
                height: draw_shape.y - Self::LOGO_SHAPE,
            };

            // begin drawing, clear screen, begin window frame.
            unsafe {
                BeginDrawing();
                ClearBackground(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                });
            }
            self.begin();

            // card header.
            self.card_sharp(card_shape, Window::COLOR_PRIMARY_MAIN);
            unsafe {
                DrawTextureV(
                    self.logo,
                    logo_point,
                    Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255,
                    },
                );
            }

            // button footer.
            self.point(Vector2 {
                x: 20.0,
                y: draw_shape.y - Self::LOGO_SHAPE + 24.0,
            });

            // create a new info file for a project, which doesn't exist yet.
            if self.button("New Project", KeyboardKey_KEY_ONE) {
                let path = std::env::current_dir()
                    .map_err(|e| Status::panic(&e.to_string()))
                    .unwrap();

                let project = rfd::FileDialog::new().set_directory(path).pick_folder();

                if let Some(project) = project {
                    Script::new_project(&project.display().to_string());

                    unsafe {
                        EndDrawing();
                    }
                    return Some(Status::new().await);
                }
            }

            // create a new info file for a project.
            if self.button("Load Project", KeyboardKey_KEY_TWO) {
                let path = std::env::current_dir()
                    .map_err(|e| Status::panic(&e.to_string()))
                    .unwrap();

                let project = rfd::FileDialog::new().set_directory(path).pick_folder();

                if let Some(project) = project {
                    Script::load_project(&project.display().to_string());

                    unsafe {
                        EndDrawing();
                    }
                    return Some(Status::new().await);
                }
            }

            // exit Alicia.
            if self.button("Exit Alicia", KeyboardKey_KEY_THREE) {
                return Some(Status::Closure);
            }

            unsafe {
                EndDrawing();
            }
        }

        Some(Status::Closure)
    }

    // draw failure window layout.
    pub async fn failure(&mut self, text: &str) -> Option<Status> {
        while unsafe { !WindowShouldClose() } {
            let draw_shape = Vector2 {
                x: unsafe { GetScreenWidth() as f32 },
                y: unsafe { GetScreenHeight() as f32 },
            };
            let card_shape = Rectangle {
                x: 0.0,
                y: 0.0,
                width: draw_shape.x,
                height: 48.0,
            };

            // begin drawing, clear screen, begin window frame.
            unsafe {
                BeginDrawing();
                ClearBackground(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                });
            }
            self.begin();

            // card header.
            self.card_sharp(card_shape, Window::COLOR_PRIMARY_MAIN);
            self.font(
                "Fatal Error",
                Vector2 { x: 20.0, y: 12.0 },
                Self::COLOR_TEXT_WHITE,
            );

            unsafe {
                DrawTextBoxed(
                    self.font,
                    Script::rust_to_c_string(text).unwrap().as_ptr(),
                    base::helper::Rectangle {
                        x: 20.0,
                        y: 72.0,
                        width: draw_shape.x - 40.0,
                        height: draw_shape.y - 244.0,
                    },
                    Self::TEXT_SHAPE,
                    Self::TEXT_SPACE,
                    true,
                    base::helper::Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                );
            }

            //self.font(
            //    &mut draw,
            //    text,
            //    Vector2::new(20.0, 72.0),
            //    Self::COLOR_TEXT_BLACK,
            //);

            // button footer.
            self.point(Vector2 {
                x: 20.0,
                y: draw_shape.y - 136.0,
            });

            // reload Alicia.
            if self.button("Load Project", KeyboardKey_KEY_ONE) {
                unsafe {
                    EndDrawing();
                }
                return Some(Status::new().await);
            }

            // copy report to clipboard.
            if self.button("Copy Report", KeyboardKey_KEY_TWO) {
                unsafe {
                    SetClipboardText(Script::rust_to_c_string(text).unwrap().as_ptr());
                }
            }

            // exit Alicia.
            if self.button("Exit Alicia", KeyboardKey_KEY_THREE) {
                unsafe {
                    EndDrawing();
                }
                return Some(Status::Closure);
            }

            unsafe {
                EndDrawing();
            }
        }

        Some(Status::Closure)
    }

    //================================================================

    // begin a new frame for the window.
    fn begin(&mut self) {
        self.point = Vector2 { x: 0.0, y: 0.0 };
        self.count = i32::default();
    }

    // set the draw cursor point.
    fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    // draw a card with a drop shadow (sharp).
    fn card_sharp(&self, rectangle: Rectangle, color: Color) {
        unsafe {
            DrawRectangleGradientV(
                rectangle.x as i32,
                (rectangle.y + rectangle.height) as i32,
                rectangle.width as i32,
                Self::GRADIENT_SHAPE_Y,
                Self::GRADIENT_COLOR_MAX,
                Self::GRADIENT_COLOR_MIN,
            );

            DrawRectangleRec(rectangle, color);
        }
    }

    // draw a card with a drop shadow (round).
    fn card_round(&self, rectangle: Rectangle, color: Color) {
        unsafe {
            DrawRectangleGradientV(
                rectangle.x as i32,
                (rectangle.y + rectangle.height - Self::GRADIENT_POINT_Y) as i32,
                rectangle.width as i32,
                Self::GRADIENT_SHAPE_Y + Self::GRADIENT_POINT_Y as i32,
                Self::GRADIENT_COLOR_MAX,
                Self::GRADIENT_COLOR_MIN,
            );

            DrawRectangleRounded(
                rectangle,
                Self::CARD_ROUND_SHAPE,
                Self::CARD_ROUND_COUNT,
                color,
            );
        }
    }

    // draw a button.
    fn button(&mut self, text: &str, key: u32) -> bool {
        // get the point and shape of the gizmo.
        let rectangle = Rectangle {
            x: self.point.x,
            y: self.point.y,
            width: Self::BUTTON_SHAPE.x,
            height: Self::BUTTON_SHAPE.y,
        };

        // get location of text.
        let text_point = Vector2 {
            x: rectangle.x + Self::BUTTON_TEXT_SHIFT.x,
            y: rectangle.y + Self::BUTTON_TEXT_SHIFT.y,
        };

        // draw card and text.
        self.card_round(rectangle, Window::COLOR_PRIMARY_MAIN);
        self.font(text, text_point, Self::COLOR_TEXT_WHITE);

        // increment the point of the next gizmo.
        self.point.y += Self::BUTTON_SHAPE.y + Self::BUTTON_SHIFT;
        self.count += 1;

        unsafe { IsKeyPressed(key as i32) }
    }

    // draw text.
    fn font(&self, text: &str, point: Vector2, color: Color) {
        unsafe {
            DrawTextEx(
                self.font,
                Script::rust_to_c_string(text).unwrap().as_ptr(),
                point,
                Self::TEXT_SHAPE,
                Self::TEXT_SPACE,
                color,
            );
        }
    }
}
