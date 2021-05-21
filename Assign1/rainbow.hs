-- import RainbowAssign
import System.Random
import qualified Data.Map as Map

-- may change at any time
-- ##################################################################
pwLength, nLetters, width, height :: Int
filename :: FilePath
pwLength = 8            -- length of each password
nLetters = 5            -- number of letters to use in passwords: 5 -> a-e
width = 40              -- length of each chain in the table
height = 1000           -- number of "rows" in the table
filename = "table.txt"  -- filename to store the table
-- ##################################################################

-- the left padding of extra 0's is taken care of in the function that takes the n least significant digits
convertBase hash base = convertBaseHelper hash base []
    where 
        convertBaseHelper hash base a
            -- hash == 0 is the terminating case for a positive hash, hash == -1 is the terminating case for a negative hash
            | hash == 0 || hash == (-1)  = a 
            | otherwise                  = convertBaseHelper (hash `div` base) base ((hash `mod` base) : a)
 


takeLSD digits n = takeLSDHelper (reverse digits) n []
    where 
        takeLSDHelper (x:xs) n a 
            | n == 0    = a
            | xs == []  = takeLSDHelper (0:[]) (n-1) (x:a)
            | otherwise = takeLSDHelper xs (n-1) (x:a)


-- pwReduce :: Integer -> String
-- pwReduce hash = map toLetter (takeLSD (convertBase hash nLetters) pwLength)


rainbowTable :: Int -> [String] -> Map.Map Integer String
rainbowTable 0 passes = Map.fromList $ map makeTuples passes
    where 
        makeTuples pass = (1726491528, pass)

-- these following functions should replace the previous rainbowTable function once we have the pwHash and pwReduce functions

-- finalHash :: Int -> String -> Integer
-- finalHash 0 pass = pwHash pass
-- finalHash n pass = finalHash (n-1) (pwReduce $ pwHash pass)

-- rainbowTable n passes = Map.fromList $ map makeTuples passes 
--     where 
--         makeTuples pass = ((finalHash n pass), pass)

