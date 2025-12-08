valid = 0

function validateline(f)
    min = parse(Int64, readuntil(f, "-"))
    max = parse(Int64, readuntil(f, " "))
    letter = readuntil(f, ": ")[1]
    password = readline(f)
    count = 0
    for l in password
        if l == letter
            count += 1
            if count > max
                return false
            end
        end
    end
    return count >= min
end

open("2020/day/2/input.txt") do f
    while !eof(f)
        if validateline(f)
            global valid += 1
        end
    end
end

println(valid)
