include $(TOPDIR)/rules.mk

PKG_NAME := ddns-rs
PKG_VERSION := 0.0.1
PKG_RELEASE := 1

STRIP=/bin/true

include $(INCLUDE_DIR)/package.mk

define Package/ddns-rs
  SECTION:=utils
  CATEGORY:=Utilities
  TITLE:=ddns-rs
endef


define Package/ddns-rs/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) /feed/ddns-rs $(1)/usr/bin/
	$(INSTALL_BIN) ./init.d/ddns-rs $(1)/etc/init.d/ddns-rs
endef



define Build/Compile
endef

$(eval $(call BuildPackage,ddns-rs))