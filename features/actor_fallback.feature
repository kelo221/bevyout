Feature: Deterministic actor appearance fallbacks
  Required appearance failures select an explicit ordered tier while optional
  equipment failures retain actor identity and the equipment decision.

  Scenario: Complete exact appearance stays authored
    Given a Humanoid appearance for base 0x10 reference 0x20
    And exact actor assets are available
    And FaceGen is not authored
    When actor appearance fallback is resolved
    Then actor fallback level is AuthoredExact
    And actor FaceGen policy is NotAuthored
    And actor proxy kind is None

  Scenario: Unsupported authored FaceGen uses the race sex rest pose
    Given a Humanoid appearance for base 0x10 reference 0x20
    And exact actor assets are available
    And race sex actor assets are available
    And FaceGen is authored but incompatible
    When actor appearance fallback is resolved
    Then actor fallback level is RaceSexSpecific
    And actor FaceGen policy is RestPoseFallback
    And actor fallback reason "missing_facegen" is recorded

  Scenario: Missing sex-specific assets use deterministic race defaults
    Given a Humanoid appearance for base 0x10 reference 0x20
    And race default actor assets are available
    When actor appearance fallback is resolved
    Then actor fallback level is RaceDefault
    And actor proxy kind is None

  Scenario: Missing race assets use the project-supported humanoid body
    Given a Humanoid appearance for base 0x10 reference 0x20
    And generic actor assets are available
    When actor appearance fallback is resolved
    Then actor fallback level is GenericProjectBody
    And actor proxy kind is GenericHumanoid

  Scenario: An unsupported actor keeps identity through the bounds proxy
    Given a Creature appearance for base 0x30 reference 0x40
    When actor appearance fallback is resolved
    Then actor fallback level is ProxyMesh
    And actor proxy kind is Bounds
    And fallback identity remains base 0x30 reference 0x40

  Scenario: A missing optional weapon does not downgrade an exact body
    Given a Humanoid appearance for base 0x10 reference 0x20
    And exact actor assets are available
    And FaceGen is not authored
    And actor fallback reason "missing_equipment" is supplied
    When actor appearance fallback is resolved
    Then actor fallback level is AuthoredExact
    And actor fallback reason "missing_equipment" is recorded

  Scenario: Required failure reasons remain distinct and deterministic
    Given a Humanoid appearance for base 0x10 reference 0x20
    And actor fallback reason "incompatible_skin" is supplied
    And actor fallback reason "missing_head_model" is supplied
    And actor fallback reason "missing_skeleton" is supplied
    When actor appearance fallback is resolved
    Then actor fallback level is ProxyMesh
    And actor fallback reasons are "missing_skeleton,missing_head_model,incompatible_skin"

  Scenario: Canonical FaceGen coefficients retain their exact layout
    Given canonical FaceGen coefficient payloads
    When FaceGen coefficients are decoded
    Then FaceGen coefficient policy is Authored
    And FaceGen geometry symmetric coefficient count is 50
    And FaceGen geometry asymmetric coefficient count is 30
    And FaceGen texture symmetric coefficient count is 50

  Scenario: Unsupported FaceGen payload uses the rest-pose fallback
    Given a FaceGen geometry payload with an unsupported length
    When FaceGen coefficients are decoded
    Then FaceGen coefficient policy is RestPoseFallback
    And FaceGen diagnostic "unsupported_facegen_layout" is recorded

  Scenario: Race defaults and actor traits combine deterministically
    Given canonical FaceGen race defaults
    And canonical FaceGen actor traits
    When FaceGen coefficients are decoded
    Then FaceGen coefficient policy is Authored
    And combined FaceGen geometry symmetric coefficient 0 is 3
