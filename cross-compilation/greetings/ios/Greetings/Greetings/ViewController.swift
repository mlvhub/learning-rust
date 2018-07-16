//
//  ViewController.swift
//  Greetings
//
//  Created by Miguel Lopez on 22/06/2018.
//  Copyright © 2018 Miguel Lopez. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view, typically from a nib.
        
        let rustGreetings = RustGreetings()
        print("\(rustGreetings.sayHello(to: "world"))")
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }


}

