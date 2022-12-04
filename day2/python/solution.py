from enum import Enum


class Move(Enum):
    ROCK = 1
    PAPER = 2
    SISSOR = 3


class Outcome(Enum):
    LOSE = 0
    DRAW = 3
    WIN  = 6


def _read_file(filename:str) -> [str]:
    out = []
    with open(filename, 'r') as file:
        for line in file:
            out.append(line.strip())

    return out


def read_file(filename: str) -> ([(Move, Move)], [(Move, Outcome)]):
    opp_move_map = {'A': Move.ROCK,
                    'B': Move.PAPER,
                    'C': Move.SISSOR,}

    my_move_map =  {'X': Move.ROCK,
                    'Y': Move.PAPER,
                    'Z': Move.SISSOR,}

    outcome_map =  {'X': Outcome.LOSE,
                    'Y': Outcome.DRAW,
                    'Z': Outcome.WIN,}

    part1_data = []
    part2_data = []
    data = _read_file(filename)
    for line in data:
        part1_data.append((opp_move_map[line[0]], my_move_map[line[2]]))
        part2_data.append((opp_move_map[line[0]], outcome_map[line[2]]))

    return (part1_data, part2_data)


def sim_part1(data: [(Move, Move)]) -> int:
    total_score = 0
    for (opp, me) in data:
        match (opp, me):
            case (Move.ROCK, Move.ROCK):
                score = Move.ROCK.value + Outcome.DRAW.value
                total_score += score
            case (Move.PAPER, Move.PAPER):
                score = Move.PAPER.value + Outcome.DRAW.value
                total_score += score
            case (Move.SISSOR, Move.SISSOR):
                score = Move.SISSOR.value + Outcome.DRAW.value
                total_score += score
            case (Move.ROCK, Move.SISSOR):
                score = Move.SISSOR.value + Outcome.LOSE.value
                total_score += score
            case (Move.PAPER, Move.ROCK):
                score = Move.ROCK.value + Outcome.LOSE.value
                total_score += score
            case (Move.SISSOR, Move.PAPER):
                score = Move.PAPER.value + Outcome.LOSE.value
                total_score += score
            case (Move.ROCK, Move.PAPER):
                score = Move.PAPER.value + Outcome.WIN.value
                total_score += score
            case (Move.PAPER, Move.SISSOR):
                score = Move.SISSOR.value + Outcome.WIN.value
                total_score += score
            case (Move.SISSOR, Move.ROCK):
                score = Move.ROCK.value + Outcome.WIN.value
                total_score += score
            case other:
                print('No logic')

    return total_score


def sim_part2(data: [(Move, Outcome)]) -> int:
    part2_total_score = 0
    for (opp, outcome) in data:
        match (opp, outcome):
            case (Move.ROCK, Outcome.LOSE):
                my_move = Move.SISSOR
                score = my_move.value + Outcome.LOSE.value
                part2_total_score += score
            case (Move.ROCK, Outcome.DRAW):
                my_move = Move.ROCK
                score = my_move.value + Outcome.DRAW.value
                part2_total_score += score
            case (Move.ROCK, Outcome.WIN):
                my_move = Move.PAPER
                score = my_move.value + Outcome.WIN.value
                part2_total_score += score
            case (Move.PAPER, Outcome.LOSE):
                my_move = Move.ROCK
                score = my_move.value + Outcome.LOSE.value
                part2_total_score += score
            case (Move.PAPER, Outcome.DRAW):
                my_move = Move.PAPER
                score = my_move.value + Outcome.DRAW.value
                part2_total_score += score
            case (Move.PAPER, Outcome.WIN):
                my_move = Move.SISSOR
                score = my_move.value + Outcome.WIN.value
                part2_total_score += score
            case (Move.SISSOR, Outcome.LOSE):
                my_move = Move.PAPER
                score = my_move.value + Outcome.LOSE.value
                part2_total_score += score
            case (Move.SISSOR, Outcome.DRAW):
                my_move = Move.SISSOR
                score = my_move.value + Outcome.DRAW.value
                part2_total_score += score
            case (Move.SISSOR, Outcome.WIN):
                my_move = Move.ROCK
                score = my_move.value + Outcome.WIN.value
                part2_total_score += score
            case other:
                print('No logic')

    return part2_total_score


def simulate() -> (int, int):
    (part1_data, part2_data) = read_file('input.txt')
    total_score = sim_part1(part1_data)
    part2_total_score = sim_part2(part2_data)
    return (total_score, part2_total_score)


if __name__ == '__main__':
    (part1_total, part2_total) = simulate()
    print("Part 1 total score: ", part1_total)
    print("Part 2 total score: ", part2_total)
