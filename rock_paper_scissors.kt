import java.util.Scanner
import kotlin.random.Random

fun main() {
    val choices = listOf("rock", "paper", "scissors")
    val scanner = Scanner(System.`in`)

    println("🎮 Rock, Paper, Scissors Game!")
    println("Enter rock, paper, or scissors: ")
    val userChoice = scanner.nextLine().toLowerCase()
    val computerChoice = choices[Random.nextInt(choices.size)]

    println("🖥 Computer chose: $computerChoice")

    when {
        userChoice == computerChoice -> println("🤝 It's a tie!")
        (userChoice == "rock" && computerChoice == "scissors") ||
        (userChoice == "paper" && computerChoice == "rock") ||
        (userChoice == "scissors" && computerChoice == "paper") -> println("🎉 You win!")
        else -> println("😞 You lose!")
    }
}
