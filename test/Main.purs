module Test.Main where

import Prelude

import Data.Foldable (traverse_)
import Effect (Effect)
import Effect.Console (log, logShow)
import Foreign.Object as Object
import Node.OS (arch, constants, cpus, devNull, endianness, eol, freemem, getCurrentProcessPriority, homedir, hostname, loadavg, machine, networkInterfaces, release, setCurrentProcessPriority, tmpdir, totalmem, type_, uptime, userInfoSE, version)

main :: Effect Unit
main = do
  log $ show eol
  logShow =<< arch
  -- `constants` is a Foreign object; print its keys (the port keeps the
  -- upstream intent of showing the available constant groups).
  log $ show $ Object.keys constants
  traverse_ logShow =<< cpus
  log $ show devNull
  logShow =<< endianness
  logShow =<< freemem
  logShow =<< getCurrentProcessPriority
  log =<< homedir
  log =<< hostname
  logShow =<< loadavg
  logShow =<< machine
  logShow =<< networkInterfaces
  log =<< release
  setCurrentProcessPriority 30
  log =<< tmpdir
  logShow =<< totalmem
  log =<< type_
  logShow =<< uptime
  logShow =<< userInfoSE
  log =<< version
