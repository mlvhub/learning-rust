//
//  RustGreetings.swift
//  Greetings
//
//  Created by Miguel Lopez on 22/06/2018.
//  Copyright © 2018 Miguel Lopez. All rights reserved.
//

import Foundation

class RustGreetings {
    func sayHello(to: String) -> String {
        let result = rust_greeting(to)
        let swift_result = String(cString: result!)
        rust_greeting_free(UnsafeMutablePointer(mutating: result))
        return swift_result
    }
}
