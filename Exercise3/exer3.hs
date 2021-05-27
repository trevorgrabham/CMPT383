import Data.Time.Calendar
import Data.Time.Calendar.OrdinalDate

merge :: Ord a => [a] -> [a] -> [a]
merge [] x = x
merge x [] = x 
merge (x:xs) (y:ys) 
    | x > y     = y : (merge (x:xs) ys)
    | otherwise = x : (merge xs (y:ys))

mergeSort :: Ord a => [a] -> [a]
mergeSort [] = []
mergeSort (x:[]) = [x]
mergeSort (x:y:[]) = merge [x] [y] 
mergeSort (x:xs) = merge (mergeSort (take firstLen (x:xs)))  (mergeSort (drop firstLen (x:xs)))
    where
        firstLen = (length (x:xs)) `div` 2

daysInYear :: Integer -> [Day]
daysInYear y = [jan1 .. dec31]
    where   jan1 = fromGregorian y 1 1
            dec31 = fromGregorian y 12 31

isFriday :: Day -> Bool
isFriday d = y == 5
    where 
        (_,y) = mondayStartWeek d

divisors :: Int -> [Int]
divisors n = [i | i <- [2.. (n `div` 2)], n `mod` i == 0] 

getDay :: (a, b, c) -> c
getDay (y,m,d) = d

isPrimeDay :: Day -> Bool 
isPrimeDay d = divisors (getDay (toGregorian d)) == []

primeFridays :: Integer -> [Day]
primeFridays y = filter isPrimeDay (filter isFriday (daysInYear y))