divisors :: Int -> [Int]
divisors n = [i | i <- [2.. (n `div` 2)], n `mod` i == 0] 

primes :: Int -> [Int]
primes n = [i | i <- [2.. n], divisors i == []]

pythagorean :: Int -> [(Int,Int,Int)]
pythagorean n = [(a,b,c) | c <- [1.. n], b <- [1.. (n-1)], a <- [1.. (n-1)], a^2 + b^2 == c^2, a < b]

join :: String -> [String] -> String
join _ [] = ""
join sep (x:xs) 
    | xs == []  = x 
    | otherwise = x ++ sep ++ (join sep xs)

fact' :: Int -> Int
fact' n = foldr (*) 1 [1..n]

hailstone :: Int -> Int
hailstone x 
    | x `mod` 2 == 0    = x `div` 2
    | otherwise         = 3*x+1

hailLen :: Int -> Int
hailLen n = hailTail 0 n 
    where   
        hailTail a 1 = a
        hailTail a n = hailTail (a+1) $ hailstone n