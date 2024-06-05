
skillset custom_robot<Position, Float> {

    resource {
        motion {
            state { On Off }
            initial Off
            transition {
                On -> Off
                Off -> On
            }
        }

        battery {
            state { Normal Critical }
            initial Normal
            transition {
                Normal -> Critical
                Critical -> Normal
            }
        }
    }

    // event {
    //     battery_to_critical {
    //         guard battery == Normal
    //         effect {
    //             battery -> Critical
    //         }
    //     }
    //     battery_to_normal {
    //         guard battery == Critical
    //         effect {
    //             battery -> Normal
    //         }
    //     }
    // }


    skill goto {

        input {
            target: Position
        }

        output result: Position

        precondition {
            can_move : motion == Off
            battery_normal : battery == Normal
        }

        start motion -> On

        invariant {
            in_movement {
                guard motion == On
            }
            battery_normal {
                guard battery == Normal
                effect {
                    motion -> Off
                }
            }
        }

        interrupt {
            effect {
                motion -> Off
            }
        }

        success {
            arrived {
                effect {
                    motion -> Off
                }
            }
        }

        failure {
            blocked {
                effect {
                    motion -> Off
                }
            }
        } 
    }

    skill recharge {

        output result: Float

        precondition {
            battery_normal : battery != Normal
            dont_move : motion == Off
        }

        invariant {
            not_in_movement {
                guard motion == Off
            }
        }

        success charged {
            postcondition battery_charged: battery == Normal
        }

        failure emergency {
            postcondition battery_not_charged: battery != Normal
        }
        
    }

}