use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use std::str::FromStr;
use std::env::args;
use std::process::exit;

struct Item {
    size: Point2,
    multiplier: f32,
}

struct Model {
    vecs: Vec<Item>,
    points: Vec<Point2>,
    index_start: usize,
    index_updating: usize,
    adding: bool,
    time_modifier: f32,
}

fn main() {

    if args().collect::<Vec<String>>().len() < 2 {
        println!("Você deve passar o nome do arquivo com as configurações dos vetores como parâmetro!!!");
        exit(-1);
    }
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn read_file(filename : &str) -> Vec<Item> {
    let mut result:Vec<Item> = Vec::new();

    let file_contents = std::fs::read_to_string(filename)
        .expect("Erro ao abrir arquivo!!!");

    for line in file_contents.split("\n") {
         // size: 100.0, speed: 1.0
        let splitted_line:Vec<&str> = line.split(",").collect();
        if splitted_line.len() != 2 {
            eprintln!("Erro ao ler linha do arquivo: {}", line);
            continue;

        }
        let size = f32::from_str(splitted_line[0].split(":").collect::<Vec<&str>>()[1].trim()).unwrap();
        let speed = f32::from_str(splitted_line[1].split(":").collect::<Vec<&str>>()[1].trim()).unwrap();

        result.push(Item {size: pt2(size, 0.0), multiplier: speed});
    }

    return result;
}

fn model(_app: &App) -> Model {
    let items: Vec<Item>;

    /* Very beautiful
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(50.0, 0.0), multiplier : 2.0});
    items.push(Item { size : pt2(25.0, 0.0), multiplier : -4.0});
    items.push(Item { size : pt2(12.5, 0.0), multiplier : -8.0});
    */

    /* Line
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(100.0, 0.0), multiplier : -1.0});
    */

    /* Flor
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 16.0});
    items.push(Item { size : pt2(100.0, 0.0), multiplier : -13.0});
    */

    /* Tres Pétalas
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 6.0});
    items.push(Item { size : pt2(100.0, 0.0), multiplier : -3.0});
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 1.5});
    */

    /* Fantasma
    items.push(Item { size : pt2(81.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(0.0, 64.0), multiplier : 2.0});
    items.push(Item { size : pt2(49.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(0.0, 36.0), multiplier : 4.0});
    items.push(Item { size : pt2(25.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(0.0, 16.0), multiplier : 8.0});
    items.push(Item { size : pt2(9.0, 0.0), multiplier : 1.0});
    items.push(Item { size : pt2(0.0, 4.0), multiplier : 16.0});
    */

    /* Donut
    items.push(Item { size : pt2(0.0, 100.0), multiplier : 50.0});
    items.push(Item { size : pt2(200.0, 0.0), multiplier : 1.0});
    */

    /* Square Wave
    let size = 100.0;
    let speed = 5.0;
    let n = 6;

    for i in 0..n {
        let multiplier = (i * 2 + 1) as f32;
        items.push(Item { size : pt2(size / multiplier, 0.0), multiplier : speed * multiplier});
    }
    */

    /* Triangular Wave
    let size = 120.0;
    let speed = 5.0;
    let n = 100;

    for i in 1..=n {
        let multiplier;
        if i % 2 == 0 {multiplier = i as f32}
        else {multiplier = -i as f32}
        items.push(Item { size : pt2(size / multiplier, 0.0), multiplier : speed * i as f32});
    }
    */
    /* Triangulo de Releux para os valores para o primeiro ponto maior que o segundo, várias pétalas para velocidades mais altas no primeiro
    items.push(Item { size : pt2(100.0, 0.0), multiplier : 2.0});
    items.push(Item { size : pt2(200.0,0.0), multiplier : -1.0});
    */

    // items.push(Item { size : pt2(10.0, 0.0), multiplier : 1.0});
    // items.push(Item { size : pt2(30.0,0.0), multiplier : -3.0});
    // items.push(Item { size : pt2(50.0,0.0), multiplier : 5.0});
    // items.push(Item { size : pt2(70.0,0.0), multiplier : -7.0});

    let arguments:Vec<String> = args().collect();
    items = read_file(&arguments[1]);

    let points = Vec::new();
    Model {
        vecs: items,
        points: points,
        index_start: 0,
        index_updating: 0,
        adding: true,
        time_modifier: 1.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let time = _app.time * _model.time_modifier;
    let mut point: Point2 = pt2(0.0, 0.0);
    let mut vecs_full_rotation = 0;

    for item in &mut _model.vecs {
        let new_point = item.size.rotate(item.multiplier * time);
        point += new_point;

        if _model.adding {
            /*
        Como são vetores girando, em algum momento irá começar a repetir o padrão, e quando
        a figura já tiver sido formada, não é necessário seguir adicionando pontos na curva
        caso contrário em algum momento acabaria a memória.
        A forma que eu verifico se todos estão alinhados com a posição inicial é verificando
        se neste update, todos os vetores estão completando um número inteiro de voltas
        pegando a rotação que é aplicada neste update "(item.multiplier * time).abs()" e
        dividindo por 2PI radianos que é o tamanho de uma volta completa
        Poderia ser feito apenas verificando se após a rotação o ponto resultante é igual ao
        ponto inicial sem rotação, porém esta forma não é adequada pois algumas figuras podem
        passar pelo ponto inicial mesmo ainda não tendo completado o ciclo completo
         */
            let num_voltas_abs = (item.multiplier * time).abs() / (2.0 * PI);

            /*
        Como é trabalhado com floats a comparação de 2 valores difícilmente será exata, então
        eu comparo dois valores aceitando um certo erro, se dois valores diferem pelo máximo
        o valor que eu especificar, então eu considero eles como iguais
        O epsilon é o erro máximo permitido, poderia ser um valor fixo tipo 0.00001, mas ao
        fixar o valor, para valores de rotação muito rápidos o erro é maior do que para
        valores de rotação mais lentos. Deste modo eu calculo o erro máximo em função
        da velocidade de rotação, quando a rotação é mais lenta, o erro permitido é menor
        para rotações rápidas o erro máximo permitido é maior.
         */
            let epsilon = (item.multiplier * _model.time_modifier).abs() * 0.0025 + 0.01; // Erro máximo permitido
            if num_voltas_abs.floor() > 0.0 {
                if (num_voltas_abs.floor() - num_voltas_abs).abs() < epsilon {
                    vecs_full_rotation += 1;
                }
            }
        }
    }

    if _model.adding {
        if vecs_full_rotation == _model.vecs.len() && !_model.points.is_empty() {
            _model.adding = false;
            _model.index_updating = 0;
            dbg!("\nAdding mode switched off");
        }

        _model.points.push(point);
        _model.index_start = _model.points.len() - 1;

        print!("\rPontos da curva {}", _model.points.len());
    }
    else {
        _model.points[_model.index_updating] = point;
        _model.index_updating = (_model.index_updating + 1) % _model.points.len();

        _model.index_start = (_model.index_start + 1) % _model.points.len();
    }

    if _app.keys.down.contains(&VirtualKeyCode::Up){ _model.time_modifier += 0.01;}
    if _app.keys.down.contains(&VirtualKeyCode::Down){ _model.time_modifier -= 0.01;}
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DARKKHAKI);

    let draw = _app.draw();

    let mut wave_points = Vec::new();
    let mut x = _app.window_rect().left() + 30.0;    // stating position of the wave
    let line_size = (_app.window_rect().right() - x) as usize;

    for index in 0..line_size {
        let mut i = _model.index_start as i32 - index as i32;
        if i < 0 {
            if _model.adding { break; } else {
                while i < 0 {
                    i += _model.points.len() as i32;
                }
            }
        }
        let new_ponint = pt2(x, _model.points[i as usize].y);
        x += 1.0;
        wave_points.push(new_ponint);
    }

    let fist_point_of_wave = wave_points[0];
    draw.polyline()
        .weight(1.5)
        .points(wave_points)
        .color(RED);

    if _model.adding {
        draw.polyline().weight(4.0).points(_model.points.clone());
    }
    else {
        let mut copy_points = _model.points.clone();
        copy_points.push(copy_points[0]);
        draw.polyline().weight(4.0).points(copy_points);
    }

    let time = _app.time * _model.time_modifier;
    let mut point: Point2 = pt2(0.0, 0.0);
    for item in &_model.vecs {
        let new_point = item.size.rotate(item.multiplier * time);
        let prev_point_copy = pt2(point.x, point.y);
        let new_point_copy = pt2(new_point.x, new_point.y);

        // Draw a line to next point
        draw.line()
            .weight(4.0)
            .start(prev_point_copy)
            .end(new_point_copy + prev_point_copy)
            .color(GREEN);

        point += new_point;
    }

    draw.line()
        .weight(2.0)
        .start(point)
        .end(fist_point_of_wave)
        .color(DARKRED);

    draw.ellipse()
        .xy(fist_point_of_wave)
        .radius(4.0)
        .color(DARKRED);

    draw.ellipse()
        .xy(point)
        .radius(5.0)
        .color(DARKGREEN);

    draw.to_frame(_app, &frame).unwrap();
}
