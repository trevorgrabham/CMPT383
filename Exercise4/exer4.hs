pascal :: Int -> [Int]
pascal 0 = [1]
pascal 1 = [1, 1]
pascal n = map (\(x,y) -> x+y) tuples
    where 
        prev    = pascal (n-1)
        tuples  = zip ([0] ++ prev) (prev ++ [0])

addPair :: (Int, Int) -> Int
addPair = uncurry (+)

withoutZeros :: (Eq a, Num a) => [a] -> [a]
withoutZeros = filter (\x -> x /= 0) 

findElt :: Ord a => a -> [a] -> (Maybe Int)
findElt _ [] = Nothing
findElt elt (x:xs) = findEltHelper elt (x:xs) 0
    where   
        findEltHelper elt (x:xs) acc  
            | elt == x  = Just acc
            | xs == []  = Nothing
            | otherwise = findEltHelper elt xs (acc+1)