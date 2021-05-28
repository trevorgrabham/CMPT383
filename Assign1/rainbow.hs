import RainbowAssign
import System.Random
import qualified Data.Map as Map
import qualified Data.Maybe as Maybe

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




-- ##################################################################
-- start my code



-- the left padding of extra 0's is taken care of in the function that takes the n least significant digits
convertBase :: Hash -> Int -> [Int]
-- base cases so that we don't immidiately terminate if the given hash is 0 or -1
convertBase 0 _ = [0]
convertBase (-1) _ = [(-1) `mod` nLetters]
convertBase hash base = convertBaseHelper (fromEnum hash) base []
    where 
        convertBaseHelper hash base a
            -- hash == 0 is the terminating case for a positive hash, hash == -1 is the terminating case for a negative hash
            | hash == 0 || hash == (-1)  = a 
            | otherwise                  = convertBaseHelper (hash `div` base) base ((hash `mod` base) : a)
 

takeLSD :: [Int] -> Int -> [Int]
-- we reverse the digits here so that we are taking the LSD first
takeLSD digits n = takeLSDHelper (reverse digits) n []
    where 
        takeLSDHelper (x:xs) n a 
            -- if we have already taken enough digits, just return the accumulator
            | n == 0    = a
            -- if we need to take more digits, but the list is empty, just append 0
            | xs == []  = takeLSDHelper (0:[]) (n-1) (x:a)
            | otherwise = takeLSDHelper xs (n-1) (x:a)


pwReduce :: Hash -> Passwd
pwReduce hash = map toLetter (takeLSD (convertBase hash nLetters) pwLength)

finalHash :: Int -> Passwd -> Hash
-- base case, just hash what we were given once
finalHash 0 pass = pwHash pass
-- recursive case
finalHash n pass = finalHash (n-1) (pwReduce $ pwHash pass)

rainbowTable :: Int -> [Passwd] -> Map.Map Hash Passwd
rainbowTable n passes = Map.fromList $ map makeTuples passes 
    where 
        -- gets the data into the correct form, a list of tuples
        makeTuples pass = ((finalHash n pass), pass)


exploreChain :: Passwd -> Int -> Hash -> Maybe Passwd
exploreChain startingPw width hash 
    -- if the password we were given, hashes into the correct hash, return it
    | pwHash startingPw == hash = Just startingPw
    -- if we have exhausted the chain and have not found the password yet, return Nothing
    | width == 0                = Nothing
    -- recursive case otherwise
    | otherwise                 = exploreChain (pwReduce (pwHash startingPw)) (width - 1) hash


findPassword :: Map.Map Hash Passwd -> Int -> Hash -> Maybe Passwd
findPassword table width hash = findPasswordHelper table width hash hash
    where
        findPasswordHelper table width hash original
            -- base case, when we have reached the beginning of the chain
            -- if there is no match in the table, then return Nothing
            | width == 0 && (Map.lookup hash table == Nothing)                                  = Nothing
            -- if there was a match, we need to check to make sure that the password hashes correctly, we send it to exploreChain
            | width == 0                                                                        = exploreChain (Maybe.fromJust (Map.lookup hash table)) width original
            -- if there are not matches we recurse
            | Map.lookup hash table == Nothing                                                  = findPasswordHelper table (width - 1) (pwHash (pwReduce hash)) original    
            -- if we found a match, but it wasn't in the chain upon further investigation, we found a false positive and continue our search                  
            | exploreChain (Maybe.fromJust (Map.lookup hash table)) width original == Nothing   = findPasswordHelper table (width - 1) (pwHash (pwReduce hash)) original
            -- if we have found a match, and we found the password in the chain, return the password
            | otherwise                                                                         = exploreChain (Maybe.fromJust (Map.lookup hash table)) width original


-- ##################################################################
-- end my code

test2 :: Int -> IO ([Passwd], Int)
test2 n = do
  table <- readTable filename
  pws <- randomPasswords nLetters pwLength n
  let hs = map pwHash pws
  let result = Maybe.mapMaybe (findPassword table width) hs
  return (result, length result)

generateTable :: IO ()
generateTable = do
    table <- buildTable rainbowTable nLetters pwLength width height
    writeTable table filename