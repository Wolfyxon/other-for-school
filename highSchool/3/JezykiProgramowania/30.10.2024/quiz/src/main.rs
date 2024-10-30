use std::io::{self, Write};

struct Question {
    pub text: String,
    pub answers: Vec<Answer>
}

impl Question {
    pub fn new(text: &str, answers: Vec<Answer>) -> Self {
        Self {
            text: text.to_string(),
            answers: answers
        }
    }

    pub fn print_questions(&self) {
        for i in 0..self.answers.len() {
            let a = &self.answers[i];

            println!(" {}. {}", nth_letter(i), a.text);
        }
    }

    pub fn query_answer(&self) -> &Answer {
        let inp = input("Podaj odpowiedź").to_lowercase();

        let ch = match inp.chars().nth(0) {
            Some(ch) => ch,
            None => {
                println!("Nie podałeś odpowiedzi");
                return self.query_answer();
            }
        };

        if !ch.is_alphabetic() {
            println!("Podaj literę");
            return self.query_answer();
        }

        let idx = ((ch as u32) - 97) as usize;

        if idx >= self.answers.len() {
            println!("Odpowiedź nie istnieje");
            return self.query_answer();
        }

        return &self.answers[idx];
    }
}

struct Answer {
    pub text: String,
    pub correct: bool
}

impl Answer {
    pub fn good(text: &str) -> Self {
        Self {
            text: text.to_string(),
            correct: true
        }
    }
    pub fn bad(text: &str) -> Self {
        Self {
            text: text.to_string(),
            correct: false
        }
    }
}

fn nth_letter(index: usize) -> char {
    char::from_u32('A' as u32 + (index as u32)).unwrap()
}

fn input(query: &str) -> String {
    print!("{}: ", query);
    io::stdout().flush().expect("stdout flush failed");

    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).expect("stdin read failed");
    
    buf
}

fn main() {
    let questions = vec![
        Question::new(
            "Które z poniższych języków są kompilowane",
            vec![
                Answer::bad("JavaScript"),
                Answer::good("Rust"),
                Answer::bad("Python"),
                Answer::bad("Lua")
            ]
        ),
        Question::new(
            "Który ze znaczników HTML nie powoduje skoku do nowej linii",
            vec![
                Answer::bad("<div>"),
                Answer::bad("<p>"),
                Answer::good("<span>"),
                Answer::bad("<br>")
            ]
        ),
        Question::new(
            "Którego stylu CSS należy użyć by zmienić kolor czcionki",
            vec![
                Answer::good("color"),
                Answer::bad("background-color"),
                Answer::bad("font"),
                Answer::bad("font-color")
            ]
        ),
        Question::new(
            "Który majonez",
            vec![
                Answer::good("Winiary"),
                Answer::bad("Kielecki"),
            ]
        ),
        Question::new(
            "Który zapis w języku Rust jest prawidłowy",
            vec![
                Answer::bad("let u32 liczba = '5'"),
                Answer::bad("u32 liczba = 5;"),
                Answer::bad("let liczba: u32 = 5"),
                Answer::good("let liczba: u32 = 5;")
            ]
        ),
        Question::new(
            "Który język NIE jest statycznie typowany",
            vec![
                Answer::bad("TypeScript"),
                Answer::good("Python"),
                Answer::bad("JavaScript"),
                Answer::bad("C++")
            ]
        ),
        Question::new(
            "Ile wynosi pierwiastek kota polanego rosołem o godzinie 12:96 17 listopada",
            vec![
                Answer::good("tak"),
                Answer::good("tak"),
            ]
        ),
        Question::new(
            "Jaka rodzina systemów operacyjnych używa tej notacji ścieżek: /home/uzytkownik",
            vec![
                Answer::bad("Windows"),
                Answer::good("UNIX"),
            ]
        ),
        Question::new(
            "Której komendy git należy użyć do przywrócenia niezcommitowanych zmian",
            vec![
                Answer::bad("git commit -m undo"),
                Answer::bad("git restore"),
                Answer::good("git checkout ."),
                Answer::bad("git checkout"),
                Answer::bad("git pull")
            ]
        ),
        Question::new(
            "Co oznacza że liczba jest nie podpisana",
            vec![
                Answer::good("Może być tylko dodatnia"),
                Answer::bad("Jej zakres nie przekracza 255"),
                Answer::bad("Nie może być przekonwertowana na znak"),
                Answer::bad("Jest stała"),
            ]
        ),
    ];

    let mut points = 0;
    let max_points = questions.len();
    
    for i in 0..questions.len() {
        let q = &questions[i];

        println!("{}. {}:", i + 1, q.text);
        q.print_questions();
        
        let ans = q.query_answer();

        if ans.correct {
            println!("Dobrze!");
            points += 1;
        } else {
            println!("Źle!");
        }
    }

    let factor = (points as f32) / (max_points as f32);
    
    let max_grade = 6.0;
    let min_grade = 1.0;
    let mut grade = (factor * max_grade).round();

    if grade < min_grade {
        grade = max_grade;
    }

    println!("\nTwój wynik: {}%", (100.0 * factor).round());
    println!("Punkty: {}/{}", points, max_points);
    println!("Ocena: {}", grade);
    
}
