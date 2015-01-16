Feature: shp start

Scenario: shp start default behavior
  Given a directory named "hi"
  When I cd to "hi"
  When I successfully run `shp start`
  When I run `shp ports`
  Then the output should contain exactly:
    """
    Argh! No other ports be known to us yet!

    """
