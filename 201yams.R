calc_proba <- function(nb, dices, arg) {
    keep <- 5 - (5 - sum(dices == nb))
    res <- 0

    if (keep < arg) {
        for (n in (arg - keep):(6 - keep)) {
            res <- res + dbinom(n, 5-keep, prob = 1/6)
        }
        res <- res * 100
    }
    else
        res <- 100
    return(res)

}

calc_straight <- function(nb, args) {
    args <- sort(args, decreasing=TRUE)
    dice <- 0
    x <- 5

    while (x > 0) {
        if (nb %in% args)
            dice <- dice + 1
        nb <- nb - 1
        x <- x - 1
    }
    return (factorial(5 - dice) / (6^(5 - dice)) * 100)
}

calc_full <- function(first, second, dices) {
    keep_first <- 5 - (5 - sum(dices == first))
    keep_second <- 5 - (5 - sum(dices == second))
    if (keep_first == 3 && keep_second == 2)
        return(100)
    if (keep_first > 3)
        keep_first <- 3
    if (keep_second > 2)
        keep_second <- 2
    nb <- 5 - keep_first - keep_second
    if (nb == 1)
        return(1 / 6 * 100)
    else {
        pair <- choose(nb, 3 - keep_first)
        three <- choose(2 - keep_second, 2 - keep_second)
        return ((pair * three / (6^nb)) * 100) # nolint
    }
}

print_res <- function(str, cmd, res) {
    cat("Chances to get a", str, cmd, ":", round(res, 2), "%\n")
}

get_cmd <- function(args) {
    exec <- strsplit(args[6], "_")
    args <- as.integer(args[-6])
    nb <- as.integer(exec[[1]][2])
    if (exec[[1]][1] == "pair") {
        print_res(as.character(exec[[1]][2]), "pair", (calc_proba(nb, args, 2)))
    }
    if (exec[[1]][1] == "three") {
        print_res(as.character(exec[[1]][2]), "three-of-a-kind", calc_proba(nb, args, 3))
    }
    if (exec[[1]][1] == "four") {
        print_res(as.character(exec[[1]][2]), "four-of-a-kind", calc_proba(nb, args, 4))
    }
    if (exec[[1]][1] == "yams") {
        print_res(as.character(exec[[1]][2]), "yams", calc_proba(nb, args, 5))
    }
    if (exec[[1]][1] == "full") {
        print_res(as.character(exec[[1]][2]), paste("full of ", as.character(exec[[1]][3])), calc_full(nb, as.integer(exec[[1]][3]), args)) # nolint
    }
    if (exec[[1]][1] == "straight") {
        print_res(as.character(exec[[1]][2]), "straight", calc_straight(nb, args))
    }
}

main <- function() {
    args <- commandArgs(trailingOnly = TRUE)
    get_cmd(args)
}

main()