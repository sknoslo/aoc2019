module Day02
  ( solve
  ) where

import Text.Megaparsec

type Parser = Parsec Void Text

-- day 2
solve :: String -> IO ()
solve contents =
  let program = parseProgram contents
   in print program

data Program =
  Program
    { behind :: [Int]
    , current :: Int
    , ahead :: [Int]
    }
  deriving (Show)

initProgram :: [Int] -> Program
initProgram (current:ahead) =
  Program {behind = [], current = current, ahead = ahead}

parseProgram :: String -> Program
parseProgram = initProgram . map read . split ","
