
# Spécifications de la couche fonctionnelle

    décharge la batterie : goto passe par running
        > custom_robot_goto_state = Running
    recharge la batterie : recharge passe par running
        > custom_robot_recharge_state = Running

    déchargent sans recharger n'est pas possible
        > not ( G(not(custom_robot_recharge_state = Running)) and G(F(custom_robot_goto_state = Running)) )
        - spécifier que ça décharge la batterie n'est pas possible vu que je ne peut pas forcer un changement de la ressource batterie qui n'est pas permis par le modèle

    un skill termine sont exécution
        > custom_robot_goto_state = Running and F(custom_robot_goto_state != Running)
        > custom_robot_recharge_state = Running and F(custom_robot_recharge_state != Running)

    tout skill termine
        > (custom_robot_goto_state = Running and F(custom_robot_goto_state != Running)) and (custom_robot_recharge_state = Running and F(custom_robot_recharge_state != Running))
    
    succès de la recharge de la batterie : recharge passe par succès
        > custom_robot_recharge_state = Success

    recharger la batterie avec succès implique qu'elle est dasn son état d'énergie maximale
        > (custom_robot_recharge_state = Success) implies (custom_robot_battery = CustomRobot_Normal)

    échec de la recharge de la batterie : recharge passe par failure
        > custom_robot_recharge_state = Failure

    recharger la batterie avec échec implique qu'elle n'est pas dans son état d'énergie maximale
        > (custom_robot_recharge_state = Failure) implies (custom_robot_battery != CustomRobot_Normal)
    
    specification totale
        > (not ( G(not(custom_robot_recharge_state = Running)) and G(F(custom_robot_goto_state = Running)) )) and (custom_robot_goto_state = Running and F(custom_robot_goto_state != Running)) and (custom_robot_recharge_state = Running and F(custom_robot_recharge_state != Running)) and ((custom_robot_recharge_state = Success) implies (custom_robot_battery = CustomRobot_Normal)) and ((custom_robot_recharge_state = Failure) implies (custom_robot_battery != CustomRobot_Normal))


# Propriétés du modèle par contre-exemples

    on peut lancer goto infiniment souvent sans lancer recharge infiniment souvent
        > G(F(custom_robot_goto_state = Running)) and G(not(custom_robot_recharge_state = Running))
    
    un skill peut ne jamais terminer
        > (F(G(custom_robot_goto_state = Running))) or (F(G(custom_robot_recharge_state = Running)))
    
    recharge fini avec succès et la batterie reste dans l'état Critique (test de la post condition du succès)
        > F(custom_robot_recharge_state = Success and custom_robot_battery = CustomRobot_Critical)

    recharge fini autrement que sur un succès et la batterie passe à l'état Normal (test d'une post condition sur la recharge partielle)
        > F((custom_robot_recharge_state = Failure or custom_robot_recharge_state = Interrupted or custom_robot_recharge_state = InvariantFailure) and custom_robot_battery = CustomRobot_Normal)