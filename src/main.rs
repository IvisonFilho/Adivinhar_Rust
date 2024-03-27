use rand::Rng;
use std::io::Write;


fn main() -> std::io::Result<()> { 
    let numero = rand::thread_rng().gen_range(0i32..=100);
    let mut _tentativas = 0;
    loop{
        print!("Digite o valor desejado para adivinhar: ");
        let _ = std::io::stdout().flush();
        let mut palpite = String::new();
        std::io::stdin().read_line(&mut palpite)?;
        
        let palpite = if let Ok(num) = palpite.trim().parse::<i32>(){
            num
        }else {
            continue
        }; 

        if palpite < 0 {
            println!("Desistiu");
            break;
        }
        if palpite == numero {
            println!("Acertou!Parabéns\n");
            println!("O valor a ser adivinhado era: {palpite}");
            break;
        } else if palpite > numero {
            println!("Diminua seu palpite\n");
        } else {
            println!("Aumente seu palpite\n");
        }
    }

    Ok(())
}
