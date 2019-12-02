module Main where

import System.Environment

import Day01
import Day02

main :: IO ()
main = do
  args <- getArgs
  contents <- getContents
  case args of
    ["1"] -> Day01.solve contents
    ["2"] -> Day02.solve contents
    [day] -> putStrLn ("Day " ++ day ++ " not solved yet")
    _ -> usage

usage :: IO ()
usage = do
  putStrLn ""
  putStrLn "  usage: aoc2019 <day>"
  putStrLn ""
  putStrLn ""
  putStrLn "    example: cat input/01.txt | aoc2019 1"
  putStrLn ""
