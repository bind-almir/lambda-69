use lambda_runtime::{run, service_fn, Context, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    statusCode: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

use rand::Rng;

struct TitForTat {
    last_opponent_move: String,
}

impl TitForTat {
    fn new() -> TitForTat {
        TitForTat {
            last_opponent_move: "cooperate".to_string(),
        }
    }

    fn next_move(&mut self, opponent_move: &str) -> String {
        self.last_opponent_move = opponent_move.to_string();
        opponent_move.to_string()
    }
}

struct RandomStrategy;

impl RandomStrategy {
    fn next_move(&self) -> String {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            "cooperate".to_string()
        } else {
            "defect".to_string()
        }
    }
}

fn play_game(rounds: usize) -> String {
    let mut tit_for_tat = TitForTat::new();
    let random_strategy = RandomStrategy;
    let mut tit_for_tat_moves = Vec::new();
    let mut random_strategy_moves = Vec::new();
    let mut html_output = String::new();
    let mut tit_for_tat_score = 0;
    let mut random_strategy_score = 0;

    html_output.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>Game Theory Simulation</title>\n<style>\n");
    html_output.push_str("body { font-family: Arial, sans-serif; line-height: 1.6; margin: 20px; background-color: #f4f4f4; color: #333; }\n");
    html_output.push_str(".container { max-width: 800px; margin: auto; background: white; padding: 20px; border-radius: 10px; box-shadow: 0 0 10px rgba(0,0,0,0.1); }\n");
    html_output.push_str("h1, h2 { color: #0073e6; }\n");
    html_output.push_str("h2 { margin-top: 1.5em; }\n");
    html_output.push_str("ul { list-style-type: none; padding: 0; }\n");
    html_output
        .push_str("ul li { margin-bottom: 10px; padding-left: 20px; text-indent: -20px; }\n");
    html_output.push_str("ul li:before { content: '• '; color: #0073e6; }\n");
    html_output.push_str("table { border-collapse: collapse; width: 100%; margin-top: 20px; }\n");
    html_output.push_str("th, td { border: 1px solid black; text-align: center; padding: 8px; }\n");
    html_output.push_str(".cooperate { background-color: green; width: 20px; height: 20px; border-radius: 50%; display: inline-block; }\n");
    html_output.push_str(".defect { background-color: red; width: 20px; height: 20px; border-radius: 50%; display: inline-block; }\n");
    html_output.push_str("</style>\n</head>\n<body>\n");
    html_output.push_str(&game_theory_html());
    html_output
        .push_str("<div class=\"container\">\n<h1>Iterated Prisoner's Dilemma Simulation</h1>\n");

    html_output.push_str("<p>I intentionally forgive the first Random Strategy move to make the game sometimes imbalanced. \n");
    html_output.push_str("This is a demo for the lightning talk on Game Theory with a sprinkle of Rust and serverless.</p> \n");
    html_output.push_str("The source code for this demo is available on <a href=\"https://github.com/bind-almir/lambda-69\">GitHub</a>.</p>\n");
    html_output.push_str(
        "<table>\n<tr><th>Round</th><th>Random Strategy</th><th>Tit-for-Tat Strategy</th></tr>\n",
    );

    for i in 0..rounds {
        let random_move = random_strategy.next_move();
        let tft_move = if i == 0 {
            // I am intentionally forgiving the first Random Strategy move to make the game imbalanced sometimes
            "cooperate".to_string()
        } else {
            tit_for_tat.next_move(&random_move)
        };

        tit_for_tat_moves.push(tft_move.clone());
        random_strategy_moves.push(random_move.clone());

        let (tft_points, random_points) = match (tft_move.as_str(), random_move.as_str()) {
            ("cooperate", "cooperate") => (3, 3),
            ("defect", "defect") => (1, 1),
            ("cooperate", "defect") => (0, 5),
            ("defect", "cooperate") => (5, 0),
            _ => (0, 0),
        };

        tit_for_tat_score += tft_points;
        random_strategy_score += random_points;

        html_output.push_str(&format!("<tr><td>{}</td><td><div class=\"{}\"></div></td><td><div class=\"{}\"></div></td></tr>\n",
            i + 1,
            if random_move == "cooperate" { "cooperate" } else { "defect" },
            if tft_move == "cooperate" { "cooperate" } else { "defect" }
        ));
    }

    html_output.push_str(&format!(
        "<h3>Scores:</h3>\n<p>Tit-for-Tat: {}</p>\n<p>Random Strategy: {}</p>\n",
        tit_for_tat_score, random_strategy_score
    ));

    html_output.push_str("</table>\n");
    html_output.push_str("</body>\n</html>");

    html_output
}

fn game_theory_html() -> String {
    let html = r#"<style>
                        body {
                            font-family: Arial, sans-serif;
                            line-height: 1.6;
                            margin: 20px;
                            background-color: #f4f4f4;
                            color: #333;
                        }
                        .container {
                            max-width: 800px;
                            margin: auto;
                            background: white;
                            padding: 20px;
                            border-radius: 10px;
                            box-shadow: 0 0 10px rgba(0,0,0,0.1);
                        }
                        h1, h2 {
                            color: #0073e6;
                        }
                        h2 {
                            margin-top: 1.5em;
                        }
                        ul {
                            list-style-type: none;
                            padding: 0;
                        }
                        ul li {
                            margin-bottom: 10px;
                            padding-left: 20px;
                            text-indent: -20px;
                        }
                        ul li:before {
                            content: "• ";
                            color: #0073e6;
                        }
                    </style>

                    <div class="container">
                        <h1>Games and Game Theory</h1>
                        <p>Game theory is all about strategic decision-making where players make choices that impact each other. Think of it like a high-stakes game where the goal is to outsmart your opponent. There are different types of games:</p>
                        <ul>
                            <li><strong>Cooperative Games:</strong> Players work together for a common goal.</li>
                            <li><strong>Non-Cooperative Games:</strong> Everyone is out for themselves.</li>
                            <li><strong>Zero-Sum Games:</strong> One player’s gain is another’s loss.</li>
                            <li><strong>Non-Zero-Sum Games:</strong> Players can all win or lose together.</li>
                        </ul>
                        <h2>The Nash Equilibrium</h2>
                        <p>The Nash Equilibrium is the ultimate "boring" scenario. It’s where all players are making the best possible moves, and no one can improve their situation by changing their strategy. Fun fact: With perfect moves, chess is always a draw. To win, you need your opponent to make mistakes. But in Nash Equilibrium, there are no mistakes to exploit.</p>
                        <h2>What’s the Dilemma?</h2>
                        <p>To snitch or not to snitch? The Prisoner's Dilemma shows why two rational individuals might not cooperate, even if it seems like the best choice. Here’s the setup:</p>
                        <ul>
                            <li>Two prisoners can either betray each other (defect) or stay silent (cooperate).</li>
                            <li>If both cooperate, they get a moderate sentence.</li>
                            <li>If one defects and the other cooperates, the defector goes free, and the cooperator gets a heavy sentence.</li>
                            <li>If both defect, they get a severe but not maximum sentence.</li>
                        </ul>
                        <h2>Tit for Tat == You Get What You Give!</h2>
                        <p>In the repeated Prisoner's Dilemma, "Tit for Tat" means you start by cooperating and then copy your opponent's last move.</p>
                        <ul>
                            <li>If they cooperate, you cooperate.</li>
                            <li>If they defect, you defect.</li>
                        </ul>
                        <p>It's simple, retaliates against defection immediately, and forgives if the opponent returns to cooperation. And a fun fact: if you convert "Tit for Tat" to a number, it equals 69—don’t ask me how!</p>
                        <h2>Prisoner's Dilemma in Cyber Security</h2>
                        <p>What's the connection between cybersecurity and the Prisoner's Dilemma? It's all about business decisions.</p>
                        <p>Companies must choose between:</p>
                        <ul>
                            <li><strong>Cooperate (Share Information):</strong> Both share malware data, enhancing overall security.</li>
                            <li><strong>Defect (Withhold Information):</strong> Each keeps its discoveries secret, hoping for a competitive edge.</li>
                        </ul>
                        <p>The dilemma highlights the tension between short-term gains and long-term security for everyone.</p>
                        <h2>Conclusion</h2>
                        <p>So, whether you're planning your next chess move, negotiating a business deal, or just deciding whether to share your Netflix password, remember: life's a game, and game theory is your ultimate playbook.</p>
                    </div>"#;
    html.to_string()
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    let html_content = play_game(20);

    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let response = Response {
        statusCode: 200,
        headers,
        body: html_content,
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
