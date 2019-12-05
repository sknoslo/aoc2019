module Day02
  ( solve
  ) where

import qualified Data.Vector as V
import Data.Void
import Text.Megaparsec
import Text.Megaparsec.Char
import qualified Text.Megaparsec.Char.Lexer as L

type Parser = Parsec Void String

-- day 2
solve :: String -> IO ()
solve contents =
  let program = parseProgram contents
   in print (V.sum program)

parseProgram :: String -> V.Vector Int
parseProgram contents =
  let (Right codes) = parse pIntcodes "" contents
   in V.fromList codes

pIntcodes :: Parser [Int]
pIntcodes = L.decimal `sepBy` char ','
