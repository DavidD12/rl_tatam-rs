skillset custom_robot {
    resource {
        motion {
            state { On Off }
            initial Off
            transition all
        }
    }

    skill goto {
        
        precondition {
            can_move : motion == Off
        }
        
        start motion -> On
        
        invariant {in_movement { guard motion == On }}
        
        success {arrived {effect {motion -> Off}}}
        
        failure {blocked {effect {motion -> Off}}}

        interrupt {
            interrupting true
            effect {
                motion -> Off
                battery -> Low
            }
        }
    }

    skill recharge {
        
        precondition {
            not_moving : motion == Off
        }
        
        invariant {not_in_movement { guard motion == Off }}
        
        success {charged {}}
    }
}