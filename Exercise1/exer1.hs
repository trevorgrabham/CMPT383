det a b c = b^2 - 4*a*c
quadsol1 a b c = (-b - sqrt(det a b c))/(2*a)
quadsol2 a b c = (-b + sqrt(det a b c))/(2*a)

third_a x = x!!2

third_b (_:_:x:_) = x

fact 0 = 1
fact x = x * fact (x-1)

hailstone x 
    | x `mod` 2 == 0    = x `div` 2
    | otherwise         = 3*x+1


hailLen 1 = 0
hailLen x = 1 + hailLen (hailstone x)