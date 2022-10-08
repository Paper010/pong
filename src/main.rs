use macroquad::prelude::*;


const PADDLE_SPEED:f32 = 10f32;
const BALL_SPEED:f32 = 9f32;

fn conf() -> Conf{
    Conf {
        window_title: "Pong".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}


enum GameState {
    Menu,
    Playing,
    Over,
}

enum Position {
    Left,
    Right,
    Center
}

struct Paddle {
    rect:Rect,
}

struct Ball {
    rect:Rect,
    vel:Vec2,
}


impl Paddle {
    pub fn new(pos:Position) -> Paddle {
        let pos:Vec2 = match pos {
            Position::Left => vec2(100.,screen_height()/2.),
            Position::Right => vec2(screen_width() / 2. + 550.,screen_height()/2.),
            Position::Center => vec2(screen_width()/2.,screen_height()/2.),
        }; 

        Paddle {
            rect: Rect::new(pos.x,pos.y,30.,100.),    
        }
    }

    pub fn update(&mut self) {
        if self.rect.y > screen_height()-self.rect.h {
            self.rect.y = screen_height()-self.rect.h;
        };

        if self.rect.y < 0. {
            self.rect.y = 0.;
        };
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x,self.rect.y,self.rect.w,self.rect.h,WHITE);
    }
}

impl Ball {
    pub fn new(pos:Position) -> Self {
        let pos:Vec2 = match pos {
            Position::Left => vec2(100.,screen_height()/2.),
            Position::Right => vec2(screen_width() / 2. + 550.,screen_height()/2.),
            Position::Center => vec2(screen_width()/2.,screen_height()/2.),
        };

        Self {
            rect: Rect::new(pos.x,pos.y,40.,40.),    
            vel: vec2(-1.,1.)
        }
    }

    pub fn update(&mut self) {
        self.rect.x += self.vel.x * BALL_SPEED;
        self.rect.y += self.vel.y * BALL_SPEED;
        println!("{}",self.vel.y); 
        if self.rect.y >screen_height()-self.rect.h{
            println!("over board");
            self.vel = vec2(self.vel.x,-self.vel.y*rand::gen_range(0.9,1.));
        };
        
        if self.rect.y < 0. {
        println!("AAAAAAAAAAA");
        self.vel = vec2(self.vel.x,1.*rand::gen_range(0.9,1.));
        println!("{}",self.vel.y);
        };
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x,self.rect.y,self.rect.w,self.rect.h,WHITE);
    }
}

fn handle_movement(player1:&mut Paddle, player2: &mut Paddle) {
    if is_key_down(KeyCode::W) {
        player1.rect.y -= 2. * PADDLE_SPEED;
    }else if is_key_down(KeyCode::S) {
        player1.rect.y += 2. * PADDLE_SPEED;

    };


    if is_key_down(KeyCode::Up){
        player2.rect.y -= 2. * PADDLE_SPEED;
    }else if is_key_down(KeyCode::Down) {
        player2.rect.y += 2. * PADDLE_SPEED;
    };
}

fn handle_collision2(ball:&mut Ball, paddle: &Paddle) -> bool {
    let mut vel = ball.vel;
    
    let intersection = match ball.rect.intersect(paddle.rect) {
        Some(intersection) => intersection,
        None => return false,
    };
    let ball_center = ball.rect.point() + ball.rect.size() * 0.5f32;
    let paddle_center = paddle.rect.point() * paddle.rect.size() * 0.5f32;
    let to = paddle_center - ball_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            ball.rect.y -= to_signum.y * intersection.h;
            ball.vel.y = -to_signum.y * ball.vel.y.abs();
        }
        false => {
            ball.rect.x -= to_signum.x * intersection.w;
            ball.vel.x = -to_signum.x * ball.vel.x.abs();
        }
    }
    true
}


fn handle_collision(ball:&mut Ball, paddle: &Paddle) -> bool {
    let mut vel = ball.vel;
    
    let intersection = match ball.rect.intersect(paddle.rect) {
        Some(intersection) => intersection,
        None => return false,
    };
    let ball_center = ball.rect.point() + ball.rect.size() * 0.5f32;
    let paddle_center = paddle.rect.point() * paddle.rect.size() * 0.5f32;
    let to = paddle_center - ball_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            ball.rect.y += to_signum.y * intersection.h;
            ball.vel.y = to_signum.y * ball.vel.y.abs();
        }
        false => {
            ball.rect.x += to_signum.x * intersection.w;
            ball.vel.x = to_signum.x * ball.vel.x.abs();
        }
    }
    true
}


#[macroquad::main(conf)]
async fn main() {
    let mut player1 = Paddle::new(Position::Left);
    let mut player2 = Paddle::new(Position::Right);
    let mut ball = Ball::new(Position::Center);

    let mut game_state = GameState::Menu;
    let mut winner:String = "No one".to_string();

    loop {
        clear_background(BLACK);

        match game_state {
        GameState::Menu=> {
        draw_text_ex("Pong!",screen_width()/2.-190.,screen_height()/2.,TextParams {
            font_size: 80,
            ..Default::default()
        });
        
        draw_text_ex("Press Space to start!",screen_width()/2.-290.,screen_height()/2. + 50.,TextParams {
            font_size: 70,
            ..Default::default()
        });
        
        draw_text_ex("Press Esc to exit",screen_width()/2.-290.,screen_height()/2.+100.,TextParams {
            font_size: 60,
            ..Default::default()
        });       
        
        if is_key_pressed(KeyCode::Space) {
            game_state = GameState::Playing;
        };
        
        },
        GameState::Playing => {
        ball.draw();
        ball.update();

        if ball.rect.x > screen_width() - ball.rect.w {
            winner = "Player 1".to_string();
            game_state = GameState::Over;
            
        }else if ball.rect.x < 0. {
            winner = "Player 2".to_string();
            game_state = GameState::Over;
        };

        handle_movement(&mut player1,&mut player2);
        player1.update();
        player2.update();
       
        handle_collision(&mut ball, &player1);
        handle_collision2(&mut ball, &player2);
        player1.draw();
        player2.draw();

        },
        GameState::Over => {
         
        draw_text_ex(&String::from(format!("{winner} Wins!")),screen_width()/2.-290.,screen_height()/2.,TextParams {
            font_size: 70,
            ..Default::default()
        });
        draw_text_ex("Press Space to restart",screen_width()/2.-290.,screen_height()/2. + 50.,TextParams {
            font_size: 70,
            ..Default::default()
        });
      
         if is_key_pressed(KeyCode::Space){
             ball = Ball::new(Position::Center);
             player1 = Paddle::new(Position::Left);
             player2 = Paddle::new(Position::Right);
            game_state = GameState::Playing;
        };
        },

       
        }
        

        // quit on esc 
        if is_key_pressed(KeyCode::Escape) {
            return;
        };

        next_frame().await;
    }
}
