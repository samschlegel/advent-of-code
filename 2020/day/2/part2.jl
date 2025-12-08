valid = 0

function validateline(f)
    i = parse(Int64, readuntil(f, "-"))
    j = parse(Int64, readuntil(f, " "))
    letter = readuntil(f, ": ")[1]
    password = readline(f)

    return (letter == password[i]) !== (letter == password[j])
end

open("2020/day/2/input.txt") do f
    while !eof(f)
        if validateline(f)
            global valid += 1
        end
    end
end

println(valid)
