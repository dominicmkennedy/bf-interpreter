(module
  (import "env" "log" (func $log (param i32)))
  (memory $mem 100)
  (func (export "main")
    (local $dp i32)
    (local.set $dp (i32.const 0))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (block $break
                  (loop $loop
                    (br_if
                      $break
                      (i32.eqz
                        (i32.load8_u (local.get $dp))))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (block $break
                  (loop $loop
                    (br_if
                      $break
                      (i32.eqz
                        (i32.load8_u (local.get $dp))))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                br $loop))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (block $break
                  (loop $loop
                    (br_if
                      $break
                      (i32.eqz
                        (i32.load8_u (local.get $dp))))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                br $loop))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                br $loop))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (call $log (i32.load8_u (local.get $dp)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.sub
                    (local.get $dp)
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.sub
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

                (local.set $dp
                  (i32.add
                    (local.get $dp)
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (i32.store8
                  (local.get $dp)
                  (i32.add
                    (i32.load8_u (local.get $dp))
                    (i32.const 1)))

                (call $log (i32.load8_u (local.get $dp)))

  )
)
