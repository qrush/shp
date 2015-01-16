Feature: shp start

Scenario: shp start default behavior
  Given a directory named "hi"
  When I cd to "hi"
  When I successfully run `shp start`
