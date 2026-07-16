Feature: Prepared point-shadow softness
  Prepared depth cubemaps can use Bevy's contact-hardening PCSS filter without
  changing the baked artifact or its cache identity.

  Scenario: The default source radius enables PCSS
    Given the default prepared point-shadow source radius
    When the prepared point-shadow radius policy is evaluated
    Then the prepared point-shadow radius is accepted
    And prepared point shadows use PCSS filtering

  Scenario: The supported source-radius endpoints are valid
    Given a prepared point-shadow source radius of 0 metres
    When the prepared point-shadow radius policy is evaluated
    Then the prepared point-shadow radius is accepted
    And prepared point shadows use hardware 2x2 filtering
    Given a prepared point-shadow source radius of 0.25 metres
    When the prepared point-shadow radius policy is evaluated
    Then the prepared point-shadow radius is accepted
    And prepared point shadows use PCSS filtering

  Scenario: Zero radius falls back to hard filtering
    Given a prepared point-shadow source radius of 0 metres
    When the prepared point-shadow radius policy is evaluated
    Then prepared point shadows use hardware 2x2 filtering

  Scenario: Invalid source radius is rejected
    Given a prepared point-shadow source radius of -0.01 metres
    When the prepared point-shadow radius policy is evaluated
    Then the prepared point-shadow radius is rejected

  Scenario: Source radius above the safety limit is rejected
    Given a prepared point-shadow source radius of 0.26 metres
    When the prepared point-shadow radius policy is evaluated
    Then the prepared point-shadow radius is rejected
