module Day01
  ( solve
  ) where

solve :: String -> IO ()
solve contents = do
  let masses = parseInput contents
      partOne = solvePartOne masses
      partTwo = solvePartTwo masses
  putStrLn ("Part 1: " ++ show partOne)
  putStrLn ("Part 2: " ++ show partTwo)

solvePartOne :: [Int] -> Int
solvePartOne = sum . map fuelRequired

solvePartTwo :: [Int] -> Int
solvePartTwo = sum . map fuelRequired'

fuelRequired :: Int -> Int
fuelRequired mass = mass `div` 3 - 2

fuelRequired' :: Int -> Int
fuelRequired' mass =
  let initial = fuelRequired mass
   in sum . takeWhile (> 0) . iterate fuelRequired $ initial

parseInput :: String -> [Int]
parseInput = map read . lines
