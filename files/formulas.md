
# Spécifications de la couche fonctionnelle

## Sous formules
    décharge la batterie : goto passe par running
        > custom_robot_goto_state = Running
    recharge la batterie : recharge passe par running
        > custom_robot_recharge_state = Running
    un skill termine sont exécution
        > not(F(G(custom_robot_goto_state = Running)))
        > not(F(G(custom_robot_recharge_state = Running)))

## Formules
    déchargent sans recharger n'est pas possible
        > not( F(G(custom_robot_recharge_state != Running)) and G(F(custom_robot_goto_state = Running)) )
        variante > (G(F(custom_robot_goto_state = Running))) implies F(custom_robot_battery = CustomRobot_Critical)

    tout skill termine
        > not(F(G(custom_robot_goto_state = Running))) and not(F(G(custom_robot_recharge_state = Running)))

## Conjonction des formules : spécification totale
    specification totale
        > (not( F(G(custom_robot_recharge_state != Running)) and G(F(custom_robot_goto_state = Running)) )) and (not(F(G(custom_robot_goto_state = Running))) and not(F(G(custom_robot_recharge_state = Running))))
    variante specification totale
        > ((G(F(custom_robot_goto_state = Running))) implies F(custom_robot_battery = CustomRobot_Critical)) and (not(F(G(custom_robot_goto_state = Running))) and not(F(G(custom_robot_recharge_state = Running))))


# Propriétés du modèle par contre-exemples

    on peut lancer goto infiniment souvent sans lancer recharge infiniment souvent
        > G(F(custom_robot_goto_state = Running)) and F(G(custom_robot_recharge_state != Running))
    
    un skill peut ne jamais terminer
        > ( F(G(custom_robot_goto_state = Running)) ) or ( F(G(custom_robot_recharge_state = Running)) )