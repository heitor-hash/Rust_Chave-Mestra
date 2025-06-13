#![windows_subsystem = "windows"]

use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};

mod crip;

fn remove_path_cript(path: &str) -> &str {
    let len = path.len();
    let (first, last) = path.split_at(len - 6);
    match last {
        ".cript" => first,
        _ => path,
    }
}

fn encrypt(path: &str, key: &str, term: fltk::terminal::Terminal) {
    let data = match std::fs::read(path) {
        Ok(data) => data,
        Err(_) => return,
    };
    let hash: [u8; 32] = crip::key_into_hash(key);
    let criptdata: Vec<u8> = match crip::encrypt(&data, &hash) {
        Ok(a) => a,
        Err(e) => {
            println!("Erro");
            fltk::dialog::message_default(format!("Erro ao criptografar {}", e).as_str());
            return;
        }
    };
    let new_path: String = format!("{}{}", path, ".cript");
    if let Some(a) = save_as_dialog(new_path.as_str()) {
        std::fs::write(&a, criptdata).unwrap();
        fltk::dialog::message_default(
            format!("Arquivo criptografado com sucesso em: {}", a).as_str(),
        );
        write_to_history(
            {
                let (h, m) = get_time_in_hour_min();
                format!("[{}:{}] Criptografado em: {path}\n", h, m, path = &a)
            },
            term,
        );
    }
}

fn decrypt(path: &str, key: &str, term: fltk::terminal::Terminal) {
    let criptdata: Vec<u8> = match std::fs::read(path) {
        Ok(a) => a,
        Err(_) => {
            fltk::dialog::message_default("Arquivo não encontrada");
            return;
        }
    };
    let hash: [u8; 32] = crip::key_into_hash(key);
    let data: Vec<u8> = match crip::decrypt(&criptdata, &hash) {
        Ok(data) => data,
        Err(e) => {
            fltk::dialog::message_default(format!("Senha incorreta, erro : {}", e).as_str());
            return;
        }
    };
    let path = remove_path_cript(path);
    if let Some(a) = save_as_dialog(path) {
        std::fs::write(&a, data).unwrap();
        fltk::dialog::message_default(
            format!("Arquivo descriptografado com sucesso em: {}", a).as_str(),
        );
        write_to_history(
            {
                let (h, m) = get_time_in_hour_min();
                format!("[{}:{}] Descriptografado em: {path}\n", h, m, path = &a)
            },
            term,
        );
    }
}

fn save_as_dialog(b: &str) -> Option<String> {
    rfd::FileDialog::default()
        .set_title("Selecione um arquivo")
        .set_file_name(b)
        .save_file()
        .map(|a| a.to_string_lossy().to_string())
}

fn write_to_history(s: String, mut term: fltk::terminal::Terminal) {
    term.append(&s);
}

fn get_time_in_hour_min() -> (u8, u8) {
    let now = time::OffsetDateTime::now_local().unwrap();
    let hour = now.hour();
    let minute = now.minute();
    (hour, minute)
}

fn main() {
    let app = fltk::app::App::default();
    // get screen width:
    let screen_width = fltk::app::screen_size().0;
    // get screen height:
    let screen_height = fltk::app::screen_size().1;

    let init_width = 600;
    let init_height = 300;
    let mut wind = fltk::window::Window::new(
        screen_width as i32 / 2 - init_width / 2,
        screen_height as i32 / 2 - init_height / 2,
        init_width,
        init_height,
        "Chave Mestra",
    );
    wind.resizable(&wind);

    let top_buttons_flex = fltk::group::Flex::default()
        .with_size(init_width - 50, 80)
        .with_pos(25, 25)
        .row();
    let mut encript_btn = fltk::button::Button::default().with_label("Criptografar");
    let mut decript_btn = fltk::button::Button::default().with_label("Descriptografar");
    top_buttons_flex.end();

    let flex = fltk::group::Flex::default()
        .with_size(init_width - 50, init_height - 130)
        .with_pos(25, 105)
        .column();

    let term: fltk::terminal::Terminal = fltk::terminal::Terminal::default();
    flex.end();

    encript_btn.set_callback({
        let term: fltk::terminal::Terminal = term.clone();
        move |_| {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Selecione arquivo para trancar")
                .pick_file()
            {
                if let Some(key) = fltk::dialog::input_default("Digite a senha", "") {
                    encrypt(path.to_str().unwrap(), &key, term.clone());
                }
                println!("Arquivo selecionado: {}", path.to_str().unwrap())
            } else {
                println!("Não selecionou arquivo")
            }
        }
    });

    decript_btn.set_callback({
        let term = term.clone();
        // Não usado pois é o ultimo movimento do term
        move |_| {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Selecione arquivo para destrancar")
                .pick_file()
            {
                if let Some(key) = fltk::dialog::input_default("Digite a senha", "") {
                    decrypt(path.to_str().unwrap(), &key, term.clone());
                }
                println!("Arquivo selecionado: {}", path.to_str().unwrap())
            } else {
                println!("Não selecionou arquivo")
            }
        }
    });

    wind.end();
    wind.show();

    app.run().unwrap();
}
