import RainbowAssign
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
convertBase :: Hash -> Int -> [Int]
convertBase hash base = convertBaseHelper (fromEnum hash) base []
    where 
        convertBaseHelper hash base a
            -- hash == 0 is the terminating case for a positive hash, hash == -1 is the terminating case for a negative hash
            | hash == 0 || hash == (-1)  = a 
            | otherwise                  = convertBaseHelper (hash `div` base) base ((hash `mod` base) : a)
 

takeLSD :: [Int] -> Int -> [Int]
takeLSD digits n = takeLSDHelper (reverse digits) n []
    where 
        takeLSDHelper (x:xs) n a 
            | n == 0    = a
            | xs == []  = takeLSDHelper (0:[]) (n-1) (x:a)
            | otherwise = takeLSDHelper xs (n-1) (x:a)


pwReduce :: Hash -> Passwd
pwReduce hash = map toLetter (takeLSD (convertBase hash nLetters) pwLength)

finalHash :: Int -> Passwd -> Hash
finalHash 0 pass = pwHash pass
finalHash n pass = finalHash (n-1) (pwReduce $ pwHash pass)

rainbowTable :: Int -> [Passwd] -> Map.Map Hash Passwd
rainbowTable n passes = Map.fromList $ map makeTuples passes 
    where 
        makeTuples pass = ((finalHash n pass), pass)

generateTable :: IO ()
generateTable = do
    table <- buildTable rainbowTable nLetters pwLength width height
    writeTable table filename