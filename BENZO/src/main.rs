use std::env;
use std::fs;
use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use console::style;

fn create_python_project(project_name: &str) {
    if !project_name.is_empty() {
        fs::create_dir_all(project_name).unwrap_or_else(|_| panic!("не смог создать папку проекта"));
        std::env::set_current_dir(project_name).unwrap_or_else(|_| panic!("не смог перейти в папку проекта"));
    }

    fs::create_dir("src").unwrap_or_else(|_| println!("src папка уже есть"));
    fs::create_dir("tests").unwrap_or_else(|_| println!("tests папка уже есть"));
    
    let reqs = "requests==2.31.0\npytest==7.4.3\nblack==23.11.0";
    fs::write("requirements.txt", reqs).unwrap_or_else(|_| println!("не смог создать requirements.txt"));
    
    let gitignore = "venv/\n__pycache__/\n*.pyc\n.pytest_cache/\n.env";
    fs::write(".gitignore", gitignore).unwrap_or_else(|_| println!("не смог создать .gitignore"));
    
    Command::new("python3")
        .args(["-m", "venv", "venv"])
        .output()
        .unwrap_or_else(|_| panic!("не смог создать виртуалку"));
        
    println!("\n{}", style("Python проект создан").green());
    if !project_name.is_empty() {
        println!("{} {}", style("Проект создан в папке:").green(), style(project_name).yellow());
    }
    println!("{} {}", style("Активируй виртуалку:").green(), style("source venv/bin/activate").yellow());
    println!("{} {}", style("Установи зависимости:").green(), style("pip install -r requirements.txt").yellow());
}

fn create_web_project(project_name: &str) {
    if !project_name.is_empty() {
        fs::create_dir_all(project_name).unwrap_or_else(|_| panic!("не смог создать папку проекта"));
        std::env::set_current_dir(project_name).unwrap_or_else(|_| panic!("не смог перейти в папку проекта"));
    }

    fs::create_dir("css").unwrap_or_else(|_| println!("css папка уже есть"));
    fs::create_dir("js").unwrap_or_else(|_| println!("js папка уже есть"));
    fs::create_dir("assets").unwrap_or_else(|_| println!("assets папка уже есть"));
    
    let html = r#"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Новый проект</title>
    <link rel="stylesheet" href="css/reset.css">
    <link rel="stylesheet" href="css/style.css">
</head>
<body>
    <div id="app"></div>
    <script type="module" src="js/main.js"></script>
</body>
</html>"#;
    
    let reset_css = r#"*, *::before, *::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    font-size: 62.5%;
}

body {
    font-size: 1.6rem;
    line-height: 1.5;
    text-rendering: optimizeSpeed;
}"#;
    
    let style_css = r#"body {
    font-family: system-ui, -apple-system, sans-serif;
    min-height: 100vh;
}

#app {
    max-width: 120rem;
    margin: 0 auto;
    padding: 2rem;
}"#;
    
    let main_js = r#"const app = document.getElementById('app');

const init = () => {
    console.log('App started');
};

init();"#;

    fs::write("index.html", html).unwrap_or_else(|_| println!("не смог создать index.html"));
    fs::write("css/reset.css", reset_css).unwrap_or_else(|_| println!("не смог создать reset.css"));
    fs::write("css/style.css", style_css).unwrap_or_else(|_| println!("не смог создать style.css"));
    fs::write("js/main.js", main_js).unwrap_or_else(|_| println!("не смог создать main.js"));
    
    println!("\n{}", style("Web проект создан").green());
    if !project_name.is_empty() {
        println!("{} {}", style("Проект создан в папке:").green(), style(project_name).yellow());
    }
}

fn create_tgbot_project(project_name: &str) {
    if !project_name.is_empty() {
        fs::create_dir_all(project_name).unwrap_or_else(|_| panic!("не смог создать папку проекта"));
        std::env::set_current_dir(project_name).unwrap_or_else(|_| panic!("не смог перейти в папку проекта"));
    }

    let main_py = r#"from aiogram import Bot, Dispatcher, Router
from aiogram.types import Message
from aiogram.filters import Command
import asyncio
from os import getenv
from dotenv import load_dotenv

load_dotenv()
router = Router()

@router.message(Command("start"))
async def start_cmd(message: Message):
    await message.answer("Привет! Я твой бот 🤖")

async def main():
    bot = Bot(token=getenv("BOT_TOKEN"))
    dp = Dispatcher()
    dp.include_router(router)
    await dp.start_polling(bot)

if __name__ == "__main__":
    asyncio.run(main())"#;

    let reqs = "aiogram>=3.0\npython-dotenv";
    let gitignore = "venv/\n__pycache__/\n*.pyc\n.env";
    let env = "BOT_TOKEN=твой_токен_тут";
    
    fs::write("bot.py", main_py).unwrap_or_else(|_| println!("не смог создать bot.py"));
    fs::write("requirements.txt", reqs).unwrap_or_else(|_| println!("не смог создать requirements.txt"));
    fs::write(".gitignore", gitignore).unwrap_or_else(|_| println!("не смог создать .gitignore"));
    fs::write(".env", env).unwrap_or_else(|_| println!("не смог создать .env"));
    
    Command::new("python3")
        .args(["-m", "venv", "venv"])
        .output()
        .unwrap_or_else(|_| panic!("не смог создать виртуалку"));
        
    println!("\n{}", style("Telegram бот создан").green());
    if !project_name.is_empty() {
        println!("{} {}", style("Проект создан в папке:").green(), style(project_name).yellow());
    }
    println!("\n{}", style("Как запустить:").cyan());
    println!("1. {} {}", style("Активируй виртуалку:").green(), style("source venv/bin/activate").yellow());
    println!("2. {} {}", style("Установи зависимости:").green(), style("pip install -r requirements.txt").yellow());
    println!("3. {} {}", style("Впиши свой токен в файл:").green(), style(".env").yellow());
    println!("4. {} {}", style("Запусти бота:").green(), style("python bot.py").yellow());
}

fn show_interactive_menu() {
    println!("\n{}", style("benzo [HMU] v0.1.0").cyan().bold());
    println!("{}", style("Hardcore Modular Utils").cyan());
    println!("{}", style("─".repeat(30)).dim());

    let items = vec!["Python проект", "Web проект", "Telegram бот"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Выбери тип проекта")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Введи имя проекта (или Enter для текущей папки)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    match selection {
        0 => create_python_project(&project_name),
        1 => create_web_project(&project_name),
        2 => create_tgbot_project(&project_name),
        _ => unreachable!(),
    }
}

fn show_help() {
    println!("\n{}", style("benzo [HMU] - Hardcore Modular Utils").cyan().bold());
    println!("{}", style("─".repeat(50)).dim());
    
    println!("\n{}", style("КОМАНДЫ:").yellow().bold());
    
    println!("\n{}", style("Основные:").yellow());
    println!("  {} {}", style("benzo").cyan(), style("[без аргументов]").dim());
    println!("    Запустить интерактивное меню");
    
    println!("\n{}", style("Python проекты:").yellow());
    println!("  {} {}", style("benzo python").cyan(), style("").dim());
    println!("    Создать Python проект в текущей папке");
    println!("  {} {}", style("benzo python").cyan(), style("my-project").yellow());
    println!("    Создать Python проект в папке my-project");
    println!("  {} {}", style("benzo python").cyan(), style("+dad my-project").yellow());
    println!("    То же самое, но через +dad");
    
    println!("\n{}", style("Web проекты:").yellow());
    println!("  {} {}", style("benzo web").cyan(), style("").dim());
    println!("    Создать Web проект в текущей папке");
    println!("  {} {}", style("benzo web").cyan(), style("my-site").yellow());
    println!("    Создать Web проект в папке my-site");
    println!("  {} {}", style("benzo web").cyan(), style("+dad my-site").yellow());
    println!("    То же самое, но через +dad");
    
    println!("\n{}", style("Telegram бот:").yellow());
    println!("  {} {}", style("benzo tgbot").cyan(), style("").dim());
    println!("    Создать Telegram бота в текущей папке");
    println!("  {} {}", style("benzo tgbot").cyan(), style("my-bot").yellow());
    println!("    Создать Telegram бота в папке my-bot");
    println!("  {} {}", style("benzo tgbot").cyan(), style("+dad my-bot").yellow());
    println!("    То же самое, но через +dad");
    
    println!("\n{}", style("Служебные:").yellow());
    println!("  {} {}", style("benzo +v").cyan(), style("").dim());
    println!("    Показать версию");
    println!("  {} {}", style("benzo +h").cyan(), style("").dim());
    println!("    Показать это сообщение");
    
    println!("\n{}", style("СТРУКТУРА ПРОЕКТОВ:").yellow().bold());
    
    println!("\n{}", style("Python:").yellow());
    println!("  src/           - исходники");
    println!("  tests/         - тесты");
    println!("  venv/          - виртуальное окружение");
    println!("  requirements.txt");
    println!("  .gitignore");
    
    println!("\n{}", style("Web:").yellow());
    println!("  index.html     - главная страница");
    println!("  css/           - стили");
    println!("    reset.css    - сброс стилей");
    println!("    style.css    - твои стили");
    println!("  js/            - скрипты");
    println!("    main.js      - главный скрипт");
    println!("  assets/        - картинки и т.д.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        show_interactive_menu();
        return;
    }

    match args[1].as_str() {
        "+v" => {
            println!("{}", style("\nbenzo [HMU] - Hardcore Modular Utils").cyan().bold());
            println!("{}", style("Version: 0.1.0").cyan());
        }
        "+h" => show_help(),
        "python" => {
            let project_name = if args.len() >= 3 {
                match args[2].as_str() {
                    "+dad" if args.len() >= 4 => &args[3],
                    name => name
                }
            } else {
                ""
            };
            create_python_project(project_name);
        }
        "web" => {
            let project_name = if args.len() >= 3 {
                match args[2].as_str() {
                    "+dad" if args.len() >= 4 => &args[3],
                    name => name
                }
            } else {
                ""
            };
            create_web_project(project_name);
        }
        "tgbot" => {
            let project_name = if args.len() >= 3 {
                match args[2].as_str() {
                    "+dad" if args.len() >= 4 => &args[3],
                    name => name
                }
            } else {
                ""
            };
            create_tgbot_project(project_name);
        }
        _ => {
            println!("\n{}", style("Неизвестная команда. Используй:").yellow());
            println!("  {} {}", style("benzo +h").cyan(), style("").dim());
            println!("    Показать помощь");
        }
    }
} 