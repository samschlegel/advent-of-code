# Find the two entries that sum to 2020; what do you get if you multiply them together?
using BenchmarkTools

lines = open("2020/day/1/input.txt") do file
    readlines(file)
end

entries = map(x -> parse(Int32, x), lines)

function findPairNaive(list, value)
    n = length(list)
    for i in 1:n
        x = list[i]
        for j in (i+1):n
            y = list[j]
            if x + y == value
                return (x, y)
            end
        end
    end
end

println("Naive Pair: ", prod(findPairNaive(entries, 2020)))
display(@benchmark findPairNaive(entries, 2020))
println()

function findPairSorted(sorted, value)
    # Assumes input is already sorted
    i = 1
    j = length(sorted)

    while i < j
        s = sorted[i] + sorted[j]
        if s == value
            return (sorted[i], sorted[j])
        elseif s < value
            i += 1
        elseif s > value
            j -= 1
        end
    end
end

function findPairSort(list, value)
    sorted = sort(list)
    return findPairSorted(sorted, value)
end

println("Sort Pair: ", prod(findPairSort(entries, 2020)))
display(@benchmark findPairSort(entries, 2020))
println()

function findTripleNaive(list, value)
    n = length(list)
    for i in 1:n
        x = list[i]
        for j in (i+1):n
            y = list[j]
            for k in (j+1):n
                z = list[k]
                if x + y + z == value
                    return (x, y, z)
                end
            end
        end
    end
end

println("Naive Triple: ", prod(findTripleNaive(entries, 2020)))
display(@benchmark findTripleNaive(entries, 2020))
println()

function findTripleSort(list, value)
    third = value / 3
    sorted = sort(list)
    n = length(list)
    for i in 1:n
        x = sorted[i]
        if x > third
            break
        end
        pair = findPairSorted(view(sorted, (i+1):n), value-x)
        if pair !== nothing
            return (x, pair[1], pair[2])
        end
    end
end

println("Sort Triple: ", prod(findTripleSort(entries, 2020)))
display(@benchmark findTripleSort(entries, 2020))
println()
