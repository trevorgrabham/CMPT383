import Data.Ratio

myIterate :: (a -> a) -> a -> [a]
myIterate f x = x : (map f $ myIterate f x)

mySplitAt :: Int -> [a] -> ([a], [a])
mySplitAt n (x:xs) = splitHelper n (x:xs) []
    where
        splitHelper 0 xs acc    = (acc, xs)
        splitHelper n [] acc    = (acc, [])
        splitHelper n (x:xs) acc    = splitHelper (n-1) xs (acc ++ [x])  

rationalSum :: Int -> [Ratio Int]
rationalSum n = [  x % (n-x) | x <- [1..(n-1)]]

rationalSumLowest :: Int -> [Ratio Int]
rationalSumLowest n = [ x % (n-x) | x <- [1..(n-1)], gcd x (n-x) == 1]

rationals :: [Ratio Int]
rationals = [ x | y <- [2..], x <- rationalSumLowest y]

-- split a list around a given separator value
splitAtSeparator :: Eq a => a -> [a] -> [[a]]
splitAtSeparator sep [] = []
splitAtSeparator sep content = first : splitAtSeparator sep rest
    where
    first = takeWhile (/= sep) content
    firstlen = length first
    rest = drop (firstlen+1) content

-- convert an integer-like string to an integer
readInt :: String -> Int
readInt = read

sumFile :: IO ()
sumFile = do 
    file <- readFile "input.txt"
    let lines = splitAtSeparator '\n' file
    let ints = map readInt lines
    print $ sum ints