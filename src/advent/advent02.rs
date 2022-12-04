enum RPS {
    Rock,
    Paper,
    Scissors
}

impl Advent02 for Advent<[&RPS]> {
    fn transform(line: &str) -> Vec<RPS> {
        let opponent = match line[0] {
            "A" -> Rock,
            "B" -> Paper,
            "C" -> Scissors 
        };
        let player = match line[3] {
            "X" -> Rock,
            "Y" -> Paper,
            "Z" -> Scissors 
        };
    }
    vec![opponent, player]


}