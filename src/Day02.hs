module Day02
  ( solve
  ) where

import Data.Vector
import Data.Vector.Mutable
import Data.Void
import Text.Megaparsec
import Text.Megaparsec.Char
import qualified Text.Megaparsec.Char.Lexer as L

type Parser = Parsec Void String

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

parseProgram :: String -> MVector Int Int
parseProgram contents =
  let (Right codes) = parse pIntcodes "" contents
   in fromList codes

pIntcodes :: Parser [Int]
pIntcodes = L.decimal `sepBy` char ','
