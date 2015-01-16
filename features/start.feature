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

Scenario: shp start with a name
  When I successfully run `shp start foo`
  When I cd to "foo"
  When I run `shp ports`
  Then the output should contain exactly:
    """
    Argh! No other ports be known to us yet!

    """

Scenario: shp start with a URL
  When I successfully run `shp start https://github.com/qrush/m.git`
  When I cd to "m"
  When I run `shp ports`
  Then the output should contain exactly:
    """
    origin
    """
