# Online Water Usage

## Présentation du projet

Ce projet est une application web permettant d’estimer la **consommation indirecte d’eau liée aux usages numériques** (réseaux sociaux, streaming, navigation web, jeux en ligne).

L’objectif est de **sensibiliser** aux impacts environnementaux souvent invisibles du numérique, tout en proposant une solution **sobre, performante et pédagogique**.

---

## Utilisation
### Locale
Pour utiliser localement la solution :
```bash
git clone https://github.com/laura-luc/online-water-usage.git
cd online-water-usage
docker compose up -d
```

Il suffit de se rendre à l'adresse : [http://localhost](http://localhost)

##  Démarche RSE

Ce projet a été conçu en suivant plusieurs **bonnes pratiques RSE**, tant sur le fond que sur la forme.

### Sensibilisation environnementale

* Le calculateur met en lumière la **consommation d’eau indirecte** liée :

  * aux centres de données,
  * aux réseaux,
  * aux usages quotidiens du numérique.
* Il favorise une **prise de conscience** plutôt qu’une mesure strictement comptable.
* Une page *Méthodologie* explique clairement les hypothèses et les limites.

### Transparence

* Les hypothèses de calcul sont :

  * explicitées,
  * accessibles à l’utilisateur,
  * séparées de l’outil principal.
* Le code est **open-source**.

---

## Choix techniques responsables

### Rust + WebAssembly (WASM)

* Le moteur de calcul est implémenté en **Rust** et non en javascript, ce qui apporte les avantages suivants :

  * performance,
  * sûreté,
  * économie de ressources.
* Compilation en **WebAssembly**, ce qui permet :

  * des temps d’exécution rapides,
  * une faible empreinte mémoire,
  * une exécution côté client (pas de serveur énergivore).

### Sobriété logicielle

* Interface simple, sans frameworks lourds,
* Utilisation de **Bootstrap**, sans surcouche inutile,
* Très peu de JavaScript : uniquement pour charger le module WASM,
* Conteneurisation "légère" (basée sur Alpine),
* Aucune collecte de données, aucun tracking.

### Accessibilité et efficacité

* Application responsive.
* Mode sombre (réduction de fatigue visuelle).
* Calculs effectués localement dans le navigateur.

---

## Utilité du projet

Ce projet est utile pour :

* la **sensibilisation écologique** (étudiants, grand public),
* des **démonstrations pédagogiques** autour du numérique responsable,
* illustrer des **bonnes pratiques de développement durable**,
* servir de base à des projets RSE, éducatifs ou associatifs.

Il montre qu’il est possible de :

> **concilier efficacité technique, sobriété numérique et impact positif.**

---

## Limites assumées

* Les valeurs utilisées sont des **ordres de grandeur**, pas des mesures exactes.
* Les impacts réels varient selon :

  * les régions,
  * les fournisseurs,
  * les services exacts utilisés
* Le projet vise avant tout l’**éducation et la sensibilisation**.

---

## Sécurité du projet

* Isolation (conteneurisation, multi-stage build, bonnes pratiques docker),
* Restriction de privilèges (service non-privilégié dans le conteneur),
* Utilisation de Rust/WASM.

## Conclusion

Ce projet s’inscrit dans une démarche de **numérique responsable**, en appliquant :

* des choix techniques efficients,
* une conception sobre,
* une finalité environnementale claire.

Il illustre concrètement comment le développement logiciel peut contribuer aux objectifs de la **RSE**.
